use std::{net::Ipv4Addr, path::PathBuf};

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use config::Config;
use env_logger::Env;
use serde::Deserialize;
use server::{
    application,
    infrastructure::{AuthRepo, CommandServer, NoteRepo, PlexRepo, TelegramBotRepo},
};

/// Server to serve the mini-view-web
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Config path
    #[clap(long)]
    config: PathBuf,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    #[serde(rename = "ADDR")]
    addr: String,

    #[serde(rename = "PORT")]
    port: String,

    #[serde(rename = "SECRET")]
    secret: String,

    #[serde(rename = "NOTION_ENDPOINT")]
    notion_endpoint: String,

    #[serde(rename = "NOTION_PAGE")]
    notion_page: String,

    #[serde(rename = "NOTION_KEY")]
    notion_key: String,

    #[serde(rename = "TELEGRAM_ENDPOINT")]
    telegram_endpoint: String,

    #[serde(rename = "CHAT_ID")]
    chat_id: String,

    #[serde(rename = "BOT_TOKEN")]
    bot_token: String,

    #[serde(rename = "PLEX_WEBHOOK_TOKEN")]
    plex_webhook_token: String,

    #[serde(rename = "PLEX_IGNORE_ADDRESSES")]
    plex_ignore_addresses: Vec<String>,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let args = Args::parse();
    let config_path = args.config.to_str().unwrap();

    let settings = Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()
        .expect("cannot parse config");
    let settings = settings.try_deserialize::<AppConfig>().unwrap();
    tracing::debug!("{:?}", settings);

    let addr: Ipv4Addr = settings.addr.parse().expect("cannot parse Ipv4Addr");
    let port = settings.port.parse::<u16>().expect("port must be integer");
    tracing::info!("Serving {}:{}", addr, port);

    let auth_repo = web::Data::new(AuthRepo::new(settings.secret));
    let note_repo = web::Data::new(NoteRepo::new(
        settings.notion_endpoint,
        settings.notion_page,
        settings.notion_key,
    ));
    let telegram_repo = web::Data::new(TelegramBotRepo::new(
        settings.telegram_endpoint,
        settings.chat_id,
        settings.bot_token,
    ));
    let plex_token_repo = web::Data::new(PlexRepo::new(
        settings.plex_webhook_token,
        settings.plex_ignore_addresses,
    ));
    let command_server = web::Data::new(CommandServer::new().start());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(plex_token_repo.clone())
            .app_data(note_repo.clone())
            .app_data(auth_repo.clone())
            .app_data(telegram_repo.clone())
            .app_data(command_server.clone())
            .configure(routes)
    })
    .bind((addr, port))?
    .run();

    server.await?;

    Ok(())
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            // Get daily org note or inspire quote
            .route("/notes", web::get().to(application::get_note::get_notes))
            // Websocket endpoint to listen to command server
            .route(
                "/ws_command",
                web::get().to(application::command::ws_command),
            )
            // Command endpoint with verification by HMAC SHA255 token
            .route(
                "/command/view/{view}/{timestamp}/{token}",
                web::put().to(application::command::change_view),
            )
            .route(
                "/plex/hook/{token}",
                web::post().to(application::plex_webhook::plex_webhook),
            ),
    );
}
