pub mod EnvFile {
    use dotenv::dotenv;

    use crate::error::types::Error;

    pub fn database_url() -> Result<String, Error> {
        dotenv().ok();
        match std::env::var("DATABASE_URL") {
            Ok(url) => Ok(url),
            Err(e) => Err(Error::VarError(e.to_string())),
        }
    }
}
