use std::time::Instant;

use actix::*;
use actix_web::{error::ErrorInternalServerError, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use reqwest::StatusCode;
use tracing::debug;

use crate::{
    domain::View,
    infrastructure::{ChangeView, CommandServer, WsCommandSession},
};

/// Entry point for our websocket route
pub async fn ws_command(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<CommandServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsCommandSession {
            id: 0,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

// PUT /command/view/{view}
pub async fn command_test(
    srv: web::Data<Addr<CommandServer>>,
    view: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let view = View::try_from(view.to_string())?;
    match srv.send(ChangeView { view }).await {
        Err(err) => {
            debug!("command_test err: {:?}", err);
            Err(ErrorInternalServerError(err))
        }
        Ok(_) => Ok(HttpResponse::new(StatusCode::OK)),
    }
}

// GET /command
pub async fn get_command(srv: web::Data<Addr<CommandServer>>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::new(StatusCode::OK))
}
