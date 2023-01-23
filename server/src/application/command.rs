use std::{str::FromStr, time::Instant};

use actix::*;
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    web, Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use reqwest::StatusCode;

use crate::{
    domain::View,
    infrastructure::{AuthRepo, ChangeView, CommandServer, WsCommandSession},
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

// PUT /command/view/{view}/{timestamp}/{token}
pub async fn change_view(
    auth_repo: web::Data<AuthRepo>,
    srv: web::Data<Addr<CommandServer>>,
    path: web::Path<(String, i64, String)>,
) -> Result<HttpResponse, Error> {
    let (view, timestamp, token) = path.into_inner();

    let message = format!("{view:}/{timestamp:}");
    auth_repo.verify_message(&token, &message, timestamp)?;

    let view = View::from_str(&view).map_err(ErrorBadRequest)?;
    match srv.send(ChangeView { view }).await {
        Err(err) => {
            tracing::debug!("command_test err: {:?}", err);
            Err(ErrorInternalServerError(err))
        }
        Ok(_) => Ok(HttpResponse::new(StatusCode::OK)),
    }
}
