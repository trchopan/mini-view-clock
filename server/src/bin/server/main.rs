use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use env_logger::Env;
use lazy_static::lazy_static;
use server::{
    application::{change_view, get_note_or_inspire, ws_command},
    infrastructure::{AuthRepo, CommandServer, NoteRepo},
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
    static ref NOTION_ENDPOINT: String = std::env::var("NOTION_ENDPOINT").unwrap();
    static ref ZEN_QUOTE_ENDPOINT: String = std::env::var("ZEN_QUOTE_ENDPOINT").unwrap();
    static ref TELEGRAM_ENDPOINT: String = std::env::var("TELEGRAM_ENDPOINT").unwrap();
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    dotenv::from_filename(args.env).expect("Cannot load env file");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let addr = ADDR.to_string();
    let port = PORT
        .to_string()
        .parse::<u16>()
        .expect("Port must be integer");
    tracing::info!("Serving {}:{}", addr, port);

    let auth_repo = web::Data::new(AuthRepo::new(args.secret));
    let note_repo = web::Data::new(NoteRepo::new(
        NOTION_ENDPOINT.to_string(),
        ZEN_QUOTE_ENDPOINT.to_string(),
        args.notion_page,
        args.notion_key,
    ));

    let command_server = CommandServer::new().start();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(note_repo.clone())
            .app_data(auth_repo.clone())
            // .app_data(telegram_repo.clone())
            .app_data(web::Data::new(command_server.clone()))
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
            .route("/note-or-inspire", web::get().to(get_note_or_inspire))
            // Websocket endpoint to listen to command server
            .route("/ws_command", web::get().to(ws_command))
            // Command endpoint with verification by HMAC SHA255 token
            .route(
                "/command/view/{view}/{timestamp}/{token}",
                web::put().to(change_view),
            ),
    );
}
