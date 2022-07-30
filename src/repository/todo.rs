use tokio_postgres::NoTls;

use super::connection::get_connection;
use crate::config::consts;

pub struct Todo {
    id: Option<u64>,
    title: String,
    memo: String,
    done: bool,
}

impl Todo {
    pub async fn insert() -> bool {
        let (client, connection) = get_connection(consts::DB_URL, NoTls).await;
        let sql = "INSERT INTO TODOS VALUES($1,$2,$3);";
        client.prepare(sql);
        if let Err(e) = client.query(sql, &[&"タイトル", &"メモ", &false]).await {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn 登録テスト() {
        assert!(Todo::insert().await);

        println!("RTT");
    }
}
