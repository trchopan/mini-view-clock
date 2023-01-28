use std::net::Ipv4Addr;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use env_logger::Env;
use lazy_static::lazy_static;
use server::{
    // application::{change_view, get_note_or_inspire, ws_command},
    application,
    infrastructure::{get_db_pool, AuthRepo, CommandServer, NoteRepo, PlexRepo, TelegramBotRepo},
};

/// Server to serve the mini-view-web
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Notion Task List page
    #[clap(long)]
    notion_page: String,

    /// Notion Read Key
    #[clap(long)]
    notion_key: String,

    /// Secret string
    #[clap(long)]
    secret: String,

    /// Telegram chat_id
    #[clap(long)]
    chat_id: String,

    /// Telegram bot token
    #[clap(long)]
    bot_token: String,

    /// Environment
    #[clap(long, default_value = ".env")]
    env: String,
}

lazy_static! {
    static ref ADDR: String = std::env::var("ADDR").unwrap();
    static ref PORT: String = std::env::var("PORT").unwrap();
    static ref DATABASE_URL: String = std::env::var("DATABASE_URL").unwrap();
    static ref NOTION_ENDPOINT: String = std::env::var("NOTION_ENDPOINT").unwrap();
    static ref ZEN_QUOTE_ENDPOINT: String = std::env::var("ZEN_QUOTE_ENDPOINT").unwrap();
    static ref TELEGRAM_ENDPOINT: String = std::env::var("TELEGRAM_ENDPOINT").unwrap();
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    dotenv::from_filename(args.env).expect("cannot load env file");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = get_db_pool(DATABASE_URL.to_string());

    let addr: Ipv4Addr = ADDR.to_string().parse().expect("cannot parse Ipv4Addr");
    let port = PORT
        .to_string()
        .parse::<u16>()
        .expect("port must be integer");
    tracing::info!("Serving {}:{}", addr, port);

    let auth_repo = web::Data::new(AuthRepo::new(args.secret));
    let note_repo = web::Data::new(NoteRepo::new(
        NOTION_ENDPOINT.to_string(),
        ZEN_QUOTE_ENDPOINT.to_string(),
        args.notion_page,
        args.notion_key,
    ));
    let telegram_repo = web::Data::new(TelegramBotRepo::new(
        TELEGRAM_ENDPOINT.to_string(),
        args.chat_id,
        args.bot_token,
    ));
    let plex_token_repo = web::Data::new(PlexRepo::new(pool));
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
            .route(
                "/note-or-inspire",
                web::get().to(application::get_note::get_note_or_inspire),
            )
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
                "/plex/new-token",
                web::post().to(application::plex_webhook::new_plex_token),
            )
            .route(
                "/plex/hook/{token}",
                web::post().to(application::plex_webhook::plex_webhook),
            ),
    );
}
