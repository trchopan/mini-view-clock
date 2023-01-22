use crate::domain::{Filter, Note, NotionDbQuery, NotionQueryResponse, QuerySelect, ZenQuote};
use actix_web::{error, Error};
use reqwest::header;

pub struct NoteRepo {
    notion_page: String,
    notion_key: String,
    notion_endpoint: String,
    zen_quote_endpoint: String,
}

impl NoteRepo {
    pub fn new(notion_page: String, notion_key: String) -> Self {
        let notion_endpoint = "https://api.notion.com/v1".to_string();
        let zen_quote_endpoint = "https://zenquotes.io/api".to_string();
        Self {
            notion_page,
            notion_key,
            notion_endpoint,
            zen_quote_endpoint,
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

        let client = reqwest::Client::new();

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
        client
            .post(url)
            .json(&query)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.notion_key))
            .header("Notion-Version", "2022-02-22")
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
                                if icon["type"] == "emoji".to_owned() {
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
