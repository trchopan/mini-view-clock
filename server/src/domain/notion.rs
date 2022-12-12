use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NotionDbQuery {
    pub filter: Filter,
}

#[derive(Serialize, Deserialize)]
pub struct Filter {
    pub property: String,
    pub select: QuerySelect,
}

#[derive(Serialize, Deserialize)]
pub struct QuerySelect {
    pub equals: String,
}

#[derive(Serialize, Deserialize)]
pub struct NotionQueryResponse {
    pub object: String,
    pub results: Vec<Result>,
    pub next_cursor: Option<serde_json::Value>,
    pub has_more: bool,
    #[serde(rename = "type")]
    pub notion_query_response_type: String,
    pub page: Page,
}

#[derive(Serialize, Deserialize)]
pub struct Page {}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub object: String,
    pub id: String,
    pub created_time: String,
    pub last_edited_time: String,
    pub created_by: TedBy,
    pub last_edited_by: TedBy,
    pub cover: Option<serde_json::Value>,
    pub icon: Option<serde_json::Value>,
    pub parent: Parent,
    pub archived: bool,
    pub properties: Properties,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct TedBy {
    pub object: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Parent {
    #[serde(rename = "type")]
    pub parent_type: String,
    pub database_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "Date Created")]
    pub date_created: DateCreated,
    #[serde(rename = "Status")]
    pub status: Status,
    #[serde(rename = "Date")]
    pub date: Date,
    #[serde(rename = "Name")]
    pub name: Name,
}

#[derive(Serialize, Deserialize)]
pub struct Date {
    pub id: String,
    #[serde(rename = "type")]
    pub date_type: String,
    pub date: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct DateCreated {
    pub id: String,
    #[serde(rename = "type")]
    pub date_created_type: String,
    pub created_time: String,
}

#[derive(Serialize, Deserialize)]
pub struct Name {
    pub id: String,
    #[serde(rename = "type")]
    pub name_type: String,
    pub title: Vec<Title>,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "type")]
    pub title_type: String,
    pub text: Text,
    pub annotations: Annotations,
    pub plain_text: String,
    pub href: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
pub struct Text {
    pub content: String,
    pub link: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub id: String,
    #[serde(rename = "type")]
    pub status_type: String,
    pub select: Select,
}

#[derive(Serialize, Deserialize)]
pub struct Select {
    pub id: String,
    pub name: String,
    pub color: String,
}
