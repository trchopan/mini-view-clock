use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};

pub struct CryptoPrice {
    client: reqwest::Client,
    api_key: String,
    endpoint: String,
}

impl CryptoPrice {
    pub fn new(api_key: String) -> Result<Self, String> {
        let mut client_headers = HeaderMap::new();

        let api_key_header =
            HeaderValue::from_str(api_key.as_str()).expect("cannot parse api_key header");
        client_headers.insert("X-CMC_PRO_API_KEY", api_key_header);

        let endpoint = "https://pro-api.coinmarketcap.com/v2".to_owned();
        let client = reqwest::Client::builder()
            .default_headers(client_headers)
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| format!("CryptoPrice err {}", e))?;

        Ok(CryptoPrice {
            client,
            api_key,
            endpoint,
        })
    }

    fn error_request(&self, err: reqwest::Error) {
        tracing::error!("error request coinmarketcap: {:?}", err);
    }

    pub async fn coinmarketcap_crypto_price(&self, symbols: Vec<String>) -> Result<String, ()> {
        let query = vec![
            //
            ("symbol", symbols.join(",")),
            ("convert", "USD".to_owned()),
        ];
        let url = format!(
            "{endpoint}/cryptocurrency/quotes/latest",
            endpoint = self.endpoint,
        );

        self.client
            .get(url)
            .query(&query)
            .send()
            .await
            .map_err(|err| self.error_request(err))?
            .text()
            .await
            .map_err(|err| self.error_request(err))
    }
}
