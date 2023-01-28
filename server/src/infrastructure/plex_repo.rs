use crate::{domain::PlexHookToken, schema::plex_hook_token};

use super::Pool;
use actix_web::{error, Error};
use diesel::{prelude::*, QueryDsl, RunQueryDsl};

pub struct PlexRepo {
    pool: Pool,
    ignore_addresses: Vec<String>,
}

impl PlexRepo {
    pub fn new(pool: Pool, ignore_addresses: Vec<String>) -> Self {
        let mut ignore_addresses = ignore_addresses.clone();
        ignore_addresses.sort();
        Self {
            pool,
            ignore_addresses,
        }
    }

    pub fn insert_plex_hook_token(&self) -> Result<PlexHookToken, Error> {
        let new_token = PlexHookToken::new();

        let mut conn = self.pool.get().expect("could not get db connection");
        diesel::insert_into(plex_hook_token::table)
            .values(&new_token)
            .execute(&mut conn)
            .map(|_| new_token)
            .map_err(|err| {
                tracing::error!("request db error: {:?}", err);
                error::ErrorInternalServerError("db error")
            })
    }

    pub fn select_plex_hook_token(&self, search_token: String) -> Result<PlexHookToken, Error> {
        use crate::schema::plex_hook_token::dsl::*;
        let mut conn = self.pool.get().expect("could not get db connection");
        plex_hook_token
            .filter(token.eq(search_token))
            .first::<PlexHookToken>(&mut conn)
            .map_err(|err| {
                tracing::error!("request db error: {:?}", err);
                error::ErrorNotFound("not found")
            })
    }

    pub fn check_ignore_address(&self, addr: &str) -> bool {
        self.ignore_addresses
            .binary_search(&addr.to_string())
            .is_ok()
    }
}
