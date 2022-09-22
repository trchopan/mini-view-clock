use chrono::Utc;
use diesel::{delete, prelude::*};
use diesel::{insert_into, update, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::domain::{Session, SessionDto};

pub async fn add_sessions(
    conn: &mut SqliteConnection,
    items: &Vec<Session>,
) -> Result<(), diesel::result::Error> {
    use crate::schema::sessions::dsl::*;
    let items: Vec<SessionDto> = items.iter().map(|i| SessionDto::from(i.clone())).collect();
    insert_into(sessions).values(items).execute(conn)?;
    Ok(())
}

pub async fn clear_timeout_sessions(
    conn: &mut SqliteConnection,
    timeout_len: i64,
) -> Result<(), diesel::result::Error> {
    use crate::schema::sessions::dsl::*;
    let limit_ts = Utc::now().timestamp() - timeout_len;
    delete(sessions.filter(updated_ts.lt(limit_ts as i32))).execute(conn)?;
    Ok(())
}

pub async fn list_sessions(
    conn: &mut SqliteConnection,
) -> Result<Vec<Session>, diesel::result::Error> {
    use crate::schema::sessions::dsl::*;
    sessions.load::<Session>(conn)
}

pub async fn get_session(
    conn: &mut SqliteConnection,
    id: String,
) -> Result<Session, diesel::result::Error> {
    use crate::schema::sessions::dsl::*;
    sessions.filter(hash_id.eq(id)).first::<Session>(conn)
}

pub async fn update_session_timestamp(
    conn: &mut SqliteConnection,
    id: String,
) -> Result<(), diesel::result::Error> {
    use crate::schema::sessions::dsl::*;
    update(sessions.filter(hash_id.eq(id)))
        .set(updated_ts.eq(Utc::now().timestamp() as i32))
        .execute(conn)?;
    Ok(())
}
