use actix_multipart::Multipart;
use actix_web::{error, web, Error, HttpResponse};
use futures_util::TryStreamExt as _;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    domain::PlexWebhookEvent,
    infrastructure::{AuthRepo, PlexRepo, TelegramBotRepo},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPlexTokenPayload {
    pub timestamp: i64,
    pub signature: String,
}

// POST "/plex/new-token",
pub async fn new_plex_token(
    auth_repo: web::Data<AuthRepo>,
    plex_token_repo: web::Data<PlexRepo>,
    payload: web::Json<NewPlexTokenPayload>,
) -> Result<HttpResponse, Error> {
    let message = payload.timestamp.to_string();
    auth_repo.verify_message(&payload.signature, &message, payload.timestamp)?;

    let token = plex_token_repo.insert_plex_hook_token()?;

    Ok(HttpResponse::Ok().body(token.token.to_string()))
}

// POST "/plex/hook/{token}",
pub async fn plex_webhook(
    plex_token_repo: web::Data<PlexRepo>,
    telegram_repo: web::Data<TelegramBotRepo>,
    path: web::Path<String>,
    mut multipart: Multipart,
) -> Result<HttpResponse, Error> {
    let token = path.into_inner();
    tracing::debug!("token: {:?}", token);

    let plex_hook_token = plex_token_repo.select_plex_hook_token(token)?;
    tracing::debug!("found plex_hook_token: {:?}", plex_hook_token);

    let mut payload: Option<PlexWebhookEvent> = None;

    while let Ok(Some(mut field)) = multipart.try_next().await {
        let mut data: String = "".to_owned();
        while let Some(chunk) = field.try_next().await.unwrap_or(None) {
            let s = std::str::from_utf8(&chunk).unwrap_or("");
            data.push_str(s);
        }
        payload = serde_json::from_str(&data).unwrap_or(None);
        if payload.is_some() {
            // Only take the first event payload
            break;
        }
    }

    let Some(event) = payload else { return Err(error::ErrorBadRequest("no event")); };

    let event_name = event.event.unwrap_or("unknown-event".to_owned());

    if event_name != "media.play" {
        tracing::info!("Nothing to handle for event: {}", event_name);
        return Ok(HttpResponse::new(StatusCode::OK));
    };

    // Event is "media.play"

    let meta = event.metadata.unwrap();
    tracing::debug!("Payload {:?}", meta);
    let unknown = || "<unknown>".to_string();

    let msg = format!(
        "Playing: {title} {year}\nType: {type}\nSummary: {summary}",
        title=meta.title.unwrap_or(unknown()),
        year=meta.year.map(|y| format!("({y})")).unwrap_or(unknown()),
        type=meta.metadata_type.unwrap_or(unknown()),
        summary=meta.summary.unwrap_or(unknown()),
    );

    match telegram_repo.send_markdown(msg).await {
        Err(err) => Err(error::ErrorBadRequest(err)),
        Ok(_) => Ok(HttpResponse::new(StatusCode::OK)),
    }
}
