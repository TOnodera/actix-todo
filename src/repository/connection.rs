use tokio_postgres::{tls::NoTlsStream, Client, Connection, NoTls, Socket};

pub async fn get_connection(config: &str, tls: NoTls) -> (Client, Connection<Socket, NoTlsStream>) {
    match tokio_postgres::connect(config, tls).await {
        Ok((client, connect)) => {
            return (client, connect);
        }
        Err(e) => {
            panic!("データベースの接続に失敗しました。: {}", e);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::consts;
}
