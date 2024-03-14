mod router;

use axum::{routing::get, Router};
use clap::Parser;
use config::Config;
use serde::Deserialize;
use server::application;

/// Server to serve the mini-view-web
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Host
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Port
    #[arg(long, default_value = "8080")]
    port: String,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    coinmarketcap_key: String,
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let args = Args::parse();

    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("MINICLOCK"))
        .build()
        .expect("cannot build config");
    let settings = settings.try_deserialize::<AppConfig>().unwrap_or_else(|e| {
        print!("cannot parse environemnt: {}", e);
        std::process::exit(1)
    });
    tracing::debug!("{:?}", settings);

    let coinprice_repo = application::crypto_price::CryptoPrice::new(settings.coinmarketcap_key)
        .expect("cannot create repo");

    let app = Router::new()
        .route("/healthz", get(router::healthz))
        .route("/coinprice", get(router::coinprice));

    let addr = &format!("{}:{}", args.host, args.port)
        .parse()
        .expect("Cannot parse the addr");

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
