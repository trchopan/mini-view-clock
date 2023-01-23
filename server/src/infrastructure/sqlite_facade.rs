use diesel::{r2d2::ConnectionManager, SqliteConnection};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type Connection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_db_pool(database_url: String) -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("cannot create db_pool")
}
