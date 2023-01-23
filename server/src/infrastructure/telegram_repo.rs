use std::{collections::HashMap, time::Duration};

#[derive(Clone)]
pub struct TelegramBotRepo {
    chat_id: String,
    token: String,
    endpoint: String,
    client: reqwest::Client,
}

impl TelegramBotRepo {
    pub fn new(telegram_endpoint: String, chat_id: String, token: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        Self {
            chat_id,
            token,
            endpoint: telegram_endpoint,
            client,
        }
    }

    fn make_url(&self, path: String) -> String {
        format!("{}/bot{}{path}", self.endpoint, self.token)
    }

    pub async fn send_markdown(&self, s: String) -> Result<(), String> {
        let url = self.make_url("/sendMessage".to_string());
        let mut map = HashMap::new();
        map.insert("chat_id", self.chat_id.clone());
        map.insert("text", s);

        tracing::debug!("URL: {}, Msg map: {:?}", url, map);
        match self.client.post(url).json(&map).send().await {
            Err(err) => {
                tracing::error!("Error request telegram: {:?}", err);
                Err("cannot request telegram".to_string())
            }
            Ok(res) => {
                tracing::debug!("Telegram resp: {:?}", res);
                Ok(())
            }
        }
    }
}
