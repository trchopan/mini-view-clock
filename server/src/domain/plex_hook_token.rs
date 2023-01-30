use crate::schema::plex_hook_token;
use chrono::Utc;
use diesel::prelude::*;
use rand::Rng;

use super::MyBase64;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = plex_hook_token)]
pub struct PlexHookToken {
    pub id: Option<i32>,
    pub token: String,
    pub created: String,
}

impl PlexHookToken {
    pub fn new_token(len: usize) -> Self {
        let token: String = rand::thread_rng()
            .sample_iter(&MyBase64)
            .take(len)
            .map(char::from)
            .collect();
        Self {
            id: None,
            token,
            created: Utc::now().to_string(),
        }
    }
}
