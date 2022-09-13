use std::sync::{atomic::AtomicUsize, Arc};

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use env_logger::Env;
use mini_view_server::{
    application::{command_test, get_note_or_inspire, ws_command, CommandServer},
    infrastructure::NoteRepo,
};

/// Server to serve the mini-view-web
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The serving addr for the server. Default: 127.0.0.1
    #[clap(long, value_parser, default_value = "127.0.0.1")]
    addr: String,

    /// The port for the server. Default: 5001
    #[clap(long, value_parser, default_value = "5001")]
    port: u16,

    /// Daily note path
    #[clap(long, value_parser)]
    note_path: std::path::PathBuf,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Serving {}:{}", args.addr, args.port);
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let note_repo = web::Data::new(NoteRepo::new(args.note_path));

    let visitor_count = Arc::new(AtomicUsize::new(0));
    let command_server = CommandServer::new(visitor_count.clone()).start();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(note_repo.clone())
            .app_data(web::Data::from(visitor_count.clone()))
            .app_data(web::Data::new(command_server.clone()))
            .configure(routes)
    })
    .bind((args.addr, args.port))?
    .run();

    server.await?;

    Ok(())
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            //
            // Get Daily Org note
            .route("/note-or-inspire", web::get().to(get_note_or_inspire))
            //
            //
            .route("/ws_command", web::get().to(ws_command))
            //
            //
            .route("/command/view/{view}", web::put().to(command_test)),
    );
}
