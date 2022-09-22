use std::{collections::HashMap, time::Duration};

use actix::prelude::*;
use tracing::{debug, error};

use crate::{
    domain::{Session, View, TIMEOUT_DURATION_SEC},
    infrastructure::{add_sessions, get_session},
};

use super::{clear_timeout_sessions, update_session_timestamp, Pool};

#[cfg(debug_assertions)]
const CLEANUP_TIMEOUT_SESSION_INTERVAL: Duration = Duration::from_secs(15);

#[cfg(not(debug_assertions))]
const CLEANUP_TIMEOUT_SESSION_INTERVAL: Duration = Duration::from_secs(5 * 60); // 5 mins

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive()]
pub struct CommandServer {
    pool: Pool,
    sessions: HashMap<String, Recipient<Message>>,
}

impl CommandServer {
    pub fn new(pool: Pool) -> CommandServer {
        CommandServer {
            pool,
            sessions: HashMap::new(),
        }
    }

    pub fn send_all(&self, message: &str) {
        for (_, addr) in self.sessions.iter() {
            addr.do_send(Message(message.to_owned()));
        }
    }

    pub fn send_one(&self, hash_id: String, message: &str) {
        let recipient = match self.sessions.get(&hash_id) {
            None => return,
            Some(r) => r,
        };
        recipient.do_send(Message(message.to_owned()));
    }

    fn clear_interval(&self, ctx: &mut Context<Self>) {
        ctx.run_interval(CLEANUP_TIMEOUT_SESSION_INTERVAL, |act, ctx| {
            let mut conn = act
                .pool
                .get()
                .expect("couldn't get db connection from pool");
            let sessions = act.sessions.clone();
            async move {
                for (hash_id, _) in sessions {
                    match update_session_timestamp(&mut conn, hash_id.clone()).await {
                        Ok(_) => debug!("session refreshed {}", hash_id),
                        Err(err) => error!("session refresh error. Err: {:?}", err),
                    }
                }
                match clear_timeout_sessions(&mut conn, TIMEOUT_DURATION_SEC).await {
                    Ok(_) => debug!("sessions timeout cleared"),
                    Err(err) => error!("sessions timeout clear error. Err: {:?}", err),
                }
            }
            .into_actor(act)
            .wait(ctx);
        });
    }
}

impl Actor for CommandServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.clear_interval(ctx);
    }
}

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub id: String,
    pub addr: Recipient<Message>,
}

impl Handler<Connect> for CommandServer {
    type Result = String;

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) -> Self::Result {
        debug!("Ws client connected");

        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        let hash_id = msg.id.clone();
        async move {
            match get_session(&mut conn, hash_id.clone()).await {
                Ok(ses) => debug!("Found session. Updated at {}", ses.updated_ts),
                Err(err) => {
                    debug!("Not found session. Err: {:?}", err);
                    let session = Session::default();
                    match add_sessions(&mut conn, &vec![session]).await {
                        Ok(_) => debug!("Session created"),
                        Err(err) => error!("Error create session. Err: hash_id {hash_id:} {err:?}"),
                    }
                }
            };
            hash_id
        }
        .into_actor(self)
        .then(|hash_id, act, _| {
            act.sessions.insert(hash_id, msg.addr);
            fut::ready(())
        })
        .wait(ctx);

        msg.id
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}

impl Handler<Disconnect> for CommandServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        debug!("Someone disconnected");
        self.sessions.remove(&msg.id);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChangeView {
    pub id: String,
    pub view: View,
}

impl Handler<ChangeView> for CommandServer {
    type Result = ();

    fn handle(&mut self, msg: ChangeView, _: &mut Self::Context) -> Self::Result {
        self.send_one(msg.id, &format!("CmdChangeView {:?}", msg.view));
    }
}

#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct ListSessions;

impl Handler<ListSessions> for CommandServer {
    type Result = Vec<String>;

    fn handle(&mut self, _: ListSessions, _: &mut Self::Context) -> Self::Result {
        self.sessions
            .keys()
            .into_iter()
            .map(|v| v.to_string())
            .collect()
    }
}
