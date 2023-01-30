use std::time::Duration;

use crate::domain::{Filter, NoteHeader, NotionDbQuery, NotionQueryResponse, QuerySelect};
use reqwest::header;

#[derive(Debug)]
pub enum NoteRepoError {
    NotionAPIError,
}

pub struct NoteRepo {
    notion_endpoint: String,
    notion_page: String,
    notion_client: reqwest::Client,
}

impl NoteRepo {
    pub fn new(notion_endpoint: String, notion_page: String, notion_key: String) -> Self {
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
            notion_page,
            notion_client,
        }
    }

    /// Get current task list from Notion filtered by Status `Today`.
    /// If there is some tasks, convert them to HTML.
    pub async fn get_notion_tasklist(&self) -> Result<Vec<NoteHeader>, NoteRepoError> {
        let err_reqwest = |err: reqwest::Error| {
            tracing::error!("request api.notion.com: {:?}", err);
            NoteRepoError::NotionAPIError
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
            .map_err(err_reqwest)?
            .json::<NotionQueryResponse>()
            .await
            .map_err(err_reqwest)
            .map(|query_response| {
                query_response
                    .results
                    .iter()
                    .map(|r| {
                        let emoji = r.icon.as_ref().and_then(|icon| {
                            if icon["type"] == "emoji" {
                                icon["emoji"].to_string().chars().nth(1)
                            } else {
                                None
                            }
                        });
                        let plain_text =
                            r.properties.name.title.first().unwrap().plain_text.clone();
                        NoteHeader {
                            emoji,
                            content: plain_text,
                        }
                    })
                    .collect::<Vec<NoteHeader>>()
            })
    }
}
