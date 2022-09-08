use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use env_logger::Env;
use mini_view_server::{application::get_note_or_inspire, infrastructure::NoteRepo};

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
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let note_repo = web::Data::new(NoteRepo::new(args.note_path));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(note_repo.clone())
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
            .route("/note-or-inspire", web::get().to(get_note_or_inspire)),
    );
}
