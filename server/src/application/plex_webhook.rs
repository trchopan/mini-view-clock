use actix_multipart::Multipart;
use actix_web::{error, web, Error, HttpResponse};
use futures_util::TryStreamExt as _;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    domain::PlexWebhookEvent,
    infrastructure::{PlexRepo, TelegramBotRepo},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPlexTokenPayload {
    pub timestamp: i64,
    pub signature: String,
}

// POST "/plex/hook/{token}",
pub async fn plex_webhook(
    plex_repo: web::Data<PlexRepo>,
    telegram_repo: web::Data<TelegramBotRepo>,
    path: web::Path<String>,
    mut multipart: Multipart,
) -> Result<HttpResponse, Error> {
    let token = path.into_inner();
    tracing::debug!("token: {:?}", token);

    if !plex_repo.check_token(&token) {
        return Err(error::ErrorNotFound("need correct token to proceed"));
    }

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

    let event_name = event.event.unwrap_or_else(|| "unknown-event".to_owned());

    if event_name != "media.play" {
        tracing::info!("Nothing to handle for event: {}", event_name);
        return Ok(HttpResponse::new(StatusCode::OK));
    };

    // Event is "media.play"

    let meta = event.metadata.unwrap();
    tracing::debug!("Meta {:?}", meta);
    let unknown = || "<unknown>".to_string();
    let msg_playing = format!("Playing {type}: {title} {year}",
        type=meta.metadata_type.unwrap_or_else(unknown),
        title=meta.title.unwrap_or_else(unknown),
        year=meta.year.map(|y| format!("({y})")).unwrap_or_else(unknown),
    );

    let msg_summary = meta.summary.unwrap_or_else(unknown);

    let player = event.player.unwrap();
    tracing::debug!("Player {:?}", player);

    let player_address = player.public_address.unwrap_or_default();
    if plex_repo.check_ignore_address(&player_address) {
        tracing::info!("Player address {} is ignored", player_address);
        return Ok(HttpResponse::new(StatusCode::OK));
    }

    let msg_player = format!(
        "Player {title} - Address {player_address}",
        title = player.title.unwrap_or_else(unknown),
        player_address = player_address,
    );

    let msg = vec![msg_playing, msg_summary, msg_player].join("\n");

    match telegram_repo.send_markdown(msg).await {
        Err(err) => Err(error::ErrorBadRequest(err)),
        Ok(_) => Ok(HttpResponse::new(StatusCode::OK)),
    }
}
