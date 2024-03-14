use axum::extract::Query;
use serde::Deserialize;

pub async fn healthz() -> &'static str {
    "ok"
}

#[derive(Debug, Deserialize)]
pub struct CoinPriceQuery {
    symbols: String,
}

pub async fn coinprice(Query(query): Query<CoinPriceQuery>) -> &'static str {
    let symbols = query.symbols.split(',').collect::<Vec<&str>>();
    tracing::debug!(">> {:?}", symbols);
    "hi"
}
