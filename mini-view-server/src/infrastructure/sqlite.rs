use diesel::{r2d2::ConnectionManager, SqliteConnection};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type Connection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;
