use std::time::Instant;

use actix::Addr;
use actix_web::{
    error::{self, ErrorInternalServerError},
    web, Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use tracing::debug;

use crate::{
    domain::{is_valid_hash_char, Session, SessionJson, View, HASH_ID_LEN},
    infrastructure::{
        add_sessions, get_session, ChangeView, CommandServer, Pool, WsCommandSession,
    },
};

// POST /session
pub async fn create_new_session(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let new_session = Session::default();
    add_sessions(&mut conn, &vec![new_session.clone()])
        .await
        .map_err(|_| error::ErrorNotFound("cannot get session"))?;

    Ok(HttpResponse::Ok().json(SessionJson::from(new_session)))
}

// GET /ws_command/{id}
pub async fn ws_command(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<Pool>,
    id: web::Path<String>,
    srv: web::Data<Addr<CommandServer>>,
) -> Result<HttpResponse, Error> {
    if id.chars().all(is_valid_hash_char) == false {
        return Err(error::ErrorBadRequest("id must be alphanumeric"));
    }
    if id.len() != HASH_ID_LEN {
        return Err(error::ErrorBadRequest("id must be 6 characters"));
    }

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let session = get_session(&mut conn, id.to_string())
        .await
        .map_err(|_| error::ErrorNotFound("cannot get session"))?;

    ws::start(
        WsCommandSession {
            session,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

// PUT /command/{id}/{view}
pub async fn change_view(
    srv: web::Data<Addr<CommandServer>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (id, view) = path.into_inner();
    let view = View::try_from(view)?;
    match srv.send(ChangeView { id, view }).await {
        Err(err) => {
            debug!("command_test err: {:?}", err);
            Err(ErrorInternalServerError(err))
        }
        Ok(_) => Ok(HttpResponse::Ok().finish()),
    }
}
