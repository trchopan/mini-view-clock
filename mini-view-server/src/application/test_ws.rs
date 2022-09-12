use std::time::Instant;

use actix::*;
use actix_web::{error::ErrorInternalServerError, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use reqwest::StatusCode;
use tracing::debug;

use super::{session, CmdChangeView, CommandServer, View};

/// Entry point for our websocket route
pub async fn ws_command(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<CommandServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsCommandSession {
            id: 0,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

// #[put("/command/view/{line_id}")]
pub async fn command_test(
    srv: web::Data<Addr<CommandServer>>,
    view: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let view = View::try_from(view.to_string())?;
    if let Err(err) = srv.send(CmdChangeView { view }).await {
        debug!("command_test err: {:?}", err);
        Err(ErrorInternalServerError(err))
    } else {
        Ok(HttpResponse::new(StatusCode::OK))
    }
}
