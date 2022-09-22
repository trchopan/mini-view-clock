use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use tracing::{debug, info};

use crate::{domain::Session, infrastructure::command_server};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct WsCommandSession {
    /// unique session id
    pub session: Session,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,

    /// Command server
    pub addr: Addr<command_server::CommandServer>,
}

impl WsCommandSession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                debug!("Websocket Client heartbeat failed, disconnecting!");

                // notify Command server
                act.addr.do_send(command_server::Disconnect {
                    id: act.session.hash_id.clone(),
                });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsCommandSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(command_server::Connect {
                id: self.session.hash_id.clone(),
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.session.hash_id = res,

                    // something is wrong with Command server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify Command server
        self.addr.do_send(command_server::Disconnect {
            id: self.session.hash_id.clone(),
        });
        Running::Stop
    }
}

/// Handle messages from Command server, we simply send it to peer websocket
impl Handler<command_server::Message> for WsCommandSession {
    type Result = ();

    fn handle(&mut self, msg: command_server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsCommandSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        debug!("WEBSOCKET MESSAGE: {msg:?}");
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(_) => {
                // let m = text.trim();
            }
            ws::Message::Binary(_) => info!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
