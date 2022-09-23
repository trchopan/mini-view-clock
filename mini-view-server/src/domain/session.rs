use crate::schema::sessions;

use chrono::{DateTime, TimeZone, Utc};
use diesel::backend::Backend;
use diesel::{Insertable, Queryable};
use rand::{prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
pub const TIMEOUT_DURATION_SEC: i64 = 3 * 60; // 3 mins

#[cfg(not(debug_assertions))]
pub const TIMEOUT_DURATION_SEC: i64 = 5 * 24 * 60 * 60; // 5 days

pub const HASH_ID_LEN: usize = 16;
const SESSION_HASH_ID_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                                   abcdefghijklmnopqrstuvwxyz\
                                                   0123456789\
                                                   _-";

#[derive(Debug, Clone, PartialEq, Queryable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub hash_id: String,
    #[diesel(deserialize_as = MyChronoTypeLoader)]
    pub created_ts: DateTime<Utc>,
    #[diesel(deserialize_as = MyChronoTypeLoader)]
    pub updated_ts: DateTime<Utc>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            hash_id: gen_hash_id(),
            created_ts: chrono::Utc::now(),
            updated_ts: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = sessions)]
pub struct SessionDto {
    pub hash_id: String,
    pub created_ts: i32,
    pub updated_ts: i32,
}

impl From<Session> for SessionDto {
    fn from(v: Session) -> Self {
        Self {
            hash_id: v.hash_id,
            created_ts: v.created_ts.timestamp() as i32,
            updated_ts: v.updated_ts.timestamp() as i32,
        }
    }
}

pub type SessionJson = SessionDto;

impl From<SessionJson> for Session {
    fn from(v: SessionJson) -> Self {
        Self {
            hash_id: v.hash_id.clone(),
            created_ts: Utc.timestamp(v.created_ts as i64, 0),
            updated_ts: Utc.timestamp(v.updated_ts as i64, 0),
        }
    }
}

pub struct MyChronoTypeLoader(DateTime<Utc>);

impl Into<chrono::DateTime<Utc>> for MyChronoTypeLoader {
    fn into(self) -> DateTime<Utc> {
        self.0
    }
}

impl<DB, ST> Queryable<ST, DB> for MyChronoTypeLoader
where
    DB: Backend,
    i32: Queryable<ST, DB>,
{
    type Row = <i32 as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(MyChronoTypeLoader(
            <i32>::build(row).map(|ts| Utc.timestamp(ts as i64, 0))?,
        ))
    }
}

struct MyBase64Charset;

impl Distribution<u8> for MyBase64Charset {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 12;
        loop {
            let var = rng.next_u32() >> (32 - 6);
            if var < RANGE {
                return SESSION_HASH_ID_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}

pub fn gen_hash_id() -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&MyBase64Charset)
        .take(HASH_ID_LEN)
        .map(char::from)
        .collect();
    rand_string
}

pub fn is_valid_hash_char(c: char) -> bool {
    SESSION_HASH_ID_ASCII_STR_CHARSET
        .iter()
        .find(|a| char::from_u32(**a as u32).unwrap() == c)
        .is_some()
}
