use crate::{domain::PlexHookToken, schema::plex_hook_token};

use super::Pool;
use diesel::{prelude::*, QueryDsl, RunQueryDsl};

#[derive(Debug)]
pub enum PlexRepoInsertError {
    DbError,
}

#[derive(Debug)]
pub enum PlexRepoSelectError {
    NotFound,
}

pub struct PlexRepo {
    pool: Pool,
    ignore_addresses: Vec<String>,
}

impl PlexRepo {
    pub fn new(pool: Pool, ignore_addresses: Vec<String>) -> Self {
        let mut ignore_addresses = ignore_addresses;
        ignore_addresses.sort();
        Self {
            pool,
            ignore_addresses,
        }
    }

    pub fn insert_plex_hook_token(&self) -> Result<PlexHookToken, PlexRepoInsertError> {
        // TODO: Solve hardcoded token length
        let new_token = PlexHookToken::new_token(16);

        let mut conn = self.pool.get().expect("could not get db connection");
        diesel::insert_into(plex_hook_token::table)
            .values(&new_token)
            .execute(&mut conn)
            .map(|_| new_token)
            .map_err(|err| {
                tracing::error!("request db error: {:?}", err);
                // error::ErrorInternalServerError("db error")
                PlexRepoInsertError::DbError
            })
    }

    pub fn select_plex_hook_token(
        &self,
        search_token: String,
    ) -> Result<PlexHookToken, PlexRepoSelectError> {
        use crate::schema::plex_hook_token::dsl::*;
        let mut conn = self.pool.get().expect("could not get db connection");
        plex_hook_token
            .filter(token.eq(search_token))
            .first::<PlexHookToken>(&mut conn)
            .map_err(|err| {
                tracing::debug!("not found token: {:?}", err);
                PlexRepoSelectError::NotFound
            })
    }

    pub fn check_ignore_address(&self, addr: &str) -> bool {
        self.ignore_addresses
            .binary_search(&addr.to_string())
            .is_ok()
    }
}
