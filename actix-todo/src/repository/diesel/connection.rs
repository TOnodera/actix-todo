use diesel::{pg::PgConnection, r2d2::ConnectionManager};

use crate::utils;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
    let database_url = utils::EnvFile::database_url().unwrap();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("DB接続に失敗しました。")
}
