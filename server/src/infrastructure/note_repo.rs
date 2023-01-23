use std::time::Duration;

use crate::domain::{Filter, Note, NotionDbQuery, NotionQueryResponse, QuerySelect, ZenQuote};
use actix_web::{error, Error};
use reqwest::header;

pub struct NoteRepo {
    notion_endpoint: String,
    zen_quote_endpoint: String,
    notion_page: String,
    notion_client: reqwest::Client,
}

impl NoteRepo {
    pub fn new(
        notion_endpoint: String,
        zen_quote_endpoint: String,
        notion_page: String,
        notion_key: String,
    ) -> Self {
        let mut notion_headers = header::HeaderMap::new();

        notion_headers.insert(
            "Notion-Version",
            header::HeaderValue::from_static("2022-02-22"),
        );
        let bearer_token = header::HeaderValue::from_str(format!("Bearer {notion_key}").as_str())
            .expect("cannot convert notion_key to header bearer token");
        notion_headers.insert(header::AUTHORIZATION, bearer_token);

        let notion_client = reqwest::Client::builder()
            .default_headers(notion_headers)
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            notion_endpoint,
            zen_quote_endpoint,
            notion_page,
            notion_client,
        }
    }
    /// Get today inspiration quote from zenquotes.io
    pub async fn get_inspire_note(&self) -> Result<Note, Error> {
        let err_reqwest = |err: reqwest::Error| {
            tracing::error!("request zenquotes.io: {:?}", err);
            error::ErrorInternalServerError("error request zenquotes.io")
        };

        reqwest::get(format!(
            "{endpoint}/today",
            endpoint = self.zen_quote_endpoint
        ))
        .await
        .map_err(|err| err_reqwest(err))?
        .json::<ZenQuote>()
        .await
        .map_err(|err| err_reqwest(err))
        .and_then(|q| {
            let quote = q.first().unwrap();
            Ok(Note {
                content: quote.to_html(),
            })
        })
    }

    /// Get current task list from Notion filtered by Status `Today`.
    /// If there is some tasks, convert them to HTML.
    pub async fn get_notion_tasklist(&self) -> Result<Note, Error> {
        let err_reqwest = |err: reqwest::Error| {
            tracing::error!("request api.notion.com: {:?}", err);
            error::ErrorInternalServerError("error request api.notion.com")
        };

        let query = NotionDbQuery {
            filter: Filter {
                property: "Status".to_owned(),
                select: QuerySelect {
                    equals: "Today".to_owned(),
                },
            },
        };

        let url = format!(
            "{endpoint}/databases/{page}/query",
            endpoint = self.notion_endpoint,
            page = self.notion_page
        );
        self.notion_client
            .post(url)
            .json(&query)
            .send()
            .await
            .map_err(|err| err_reqwest(err))?
            .json::<NotionQueryResponse>()
            .await
            .map_err(|err| err_reqwest(err))
            .and_then(|q| {
                let headers: Vec<(Option<char>, String)> = q
                    .results
                    .iter()
                    .map(|r| {
                        let emoji = r.icon.as_ref().map_or_else(
                            || None,
                            |icon| {
                                if icon["type"] == "emoji" {
                                    icon["emoji"].to_string().chars().nth(1)
                                } else {
                                    None
                                }
                            },
                        );
                        let plain_text =
                            r.properties.name.title.first().unwrap().plain_text.clone();
                        (emoji, plain_text)
                    })
                    .collect();
                if headers.len() > 0 {
                    Ok(Note::from_headers_to_html(headers))
                } else {
                    Err(error::ErrorNotFound("empty results from notion"))
                }
            })
    }
}
