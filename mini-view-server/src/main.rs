use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use env_logger::Env;
use mini_view_server::{
    application::{
        change_view, create_new_session, get_note_or_inspire, get_sessions, get_ws_sessions,
        new_sessions, ws_command,
    },
    domain::TIMEOUT_DURATION_SEC,
    infrastructure::{CommandServer, NoteRepo, Pool},
};
use tracing::info;

/// Server to serve the mini-view-web
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Daily note path
    #[clap(long, value_parser)]
    note_path: std::path::PathBuf,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv::dotenv().ok();

    let server_addr = std::env::var("ADDR").expect("not found");
    let server_port = std::env::var("PORT")
        .expect("not found")
        .parse::<u16>()
        .expect("port not u16");
    info!("Serving {}:{}", server_addr, server_port);

    let db_url = std::env::var("DATABASE_URL").expect("not found");
    let db_manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let db_pool = Pool::builder()
        .build(db_manager)
        .expect("cannot create db_pool");

    let command_server = CommandServer::new(db_pool.clone()).start();

    let args = Args::parse();
    let note_repo = web::Data::new(NoteRepo::new(args.note_path));

    info!("TIMEOUT_DURATION_SEC: {}", TIMEOUT_DURATION_SEC);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(command_server.clone()))
            .app_data(note_repo.clone())
            .configure(debug_service) // Must before prod_service
            .configure(prod_service)
    })
    .bind((server_addr, server_port))?
    .run();

    server.await?;

    Ok(())
}

fn prod_service(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .route("/note-or-inspire", web::get().to(get_note_or_inspire))
        .route("/session", web::post().to(create_new_session))
        .route("/ws_command/{id}", web::get().to(ws_command))
        .route("/command/{id}/{view}", web::put().to(change_view));
    cfg.service(scope);
}

#[cfg(debug_assertions)]
fn debug_service(cfg: &mut web::ServiceConfig) {
    if let Err(_) = std::env::var("DEBUG") {
        return;
    }
    let scope = web::scope("/debug")
        .route("/ws_sessions", web::get().to(get_ws_sessions))
        .route("/sessions", web::get().to(get_sessions))
        .route("/sessions", web::post().to(new_sessions));
    cfg.service(scope);

    info!("Debug routes are activated");
}
