pub mod EnvFile {
    use dotenv::dotenv;

    use crate::error::types::AppError;

    pub fn database_url() -> Result<String, AppError> {
        dotenv().ok();
        match std::env::var("DATABASE_URL") {
            Ok(url) => Ok(url),
            Err(e) => Err(AppError::VarError(e.to_string())),
        }
    }
}
