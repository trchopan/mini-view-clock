use actix::Addr;
use actix_web::{error, web, Error, HttpResponse};
use tracing::debug;

use crate::{
    domain::{Session, SessionJson},
    infrastructure::{
        add_sessions, list_sessions, CommandServer, ListSessions, Pool,
    },
};

// GET /debug/ws_sessions
pub async fn get_ws_sessions(srv: web::Data<Addr<CommandServer>>) -> Result<HttpResponse, Error> {
    match srv.send(ListSessions).await {
        Err(err) => {
            debug!("command_test err: {:?}", err);
            Err(error::ErrorInternalServerError(err))
        }
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
    }
}

// GET /debug/sessions
pub async fn get_sessions(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let sessions = list_sessions(&mut conn)
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;

    let sessions: Vec<SessionJson> = sessions
        .iter()
        .map(|ses| SessionJson::from(ses.clone()))
        .collect();
    Ok(HttpResponse::Ok().json(sessions))
}

// POST /debug/sessions
pub async fn new_sessions(
    pool: web::Data<Pool>,
    sessions: web::Json<Vec<SessionJson>>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let sessions: Vec<Session> = sessions
        .0
        .iter()
        .map(|s| Session::from(s.clone()))
        .collect();
    add_sessions(&mut conn, &sessions)
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().finish())
}
