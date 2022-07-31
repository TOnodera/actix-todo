use diesel::{pg::PgConnection, Connection};

use crate::error::types::Error;

pub fn get_connection(database_url: String) -> Result<PgConnection, Error> {
    let result = PgConnection::establish(&database_url);
    match result {
        Ok(connection) => Ok(connection),
        Err(_) => Err(Error::ConnectionError(
            "データベースに接続失敗しました。接続情報を確認してください。".to_string(),
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn can_connect_database() {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let result = get_connection(database_url);
        assert!(result.is_ok());
    }

    #[test]
    fn show_error_message_when_db_connect_is_failure() {
        let is_err = get_connection("Invalid DB connection information".to_string()).map_err(|e| {
            assert!(e.to_string() == "Connection Error: データベースに接続失敗しました。接続情報を確認してください。");
        }).is_err();

        assert!(is_err);
    }
}
