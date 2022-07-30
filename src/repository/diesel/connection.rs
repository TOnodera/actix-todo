use diesel::{pg::PgConnection, Connection};
use dotenv::dotenv;

pub fn get_connection() -> PgConnection {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("接続に失敗しました。");
    PgConnection::establish(&database_url).expect("接続に失敗しました。")
}
