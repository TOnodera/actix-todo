use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    ConnectionError(String),
    DatabaseRuntimeError(String),
    VarError(String),
    NotFoundError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConnectionError(e) => {
                write!(f, "Connection Error: {}", e)
            }
            Error::DatabaseRuntimeError(e) => {
                write!(f, "DatabaseRuntime Error: {}", e)
            }
            Error::VarError(e) => {
                write!(f, "VarError: {}", e)
            }
            Error::NotFoundError(e) => {
                write!(f, "NotFoundError: {}", e)
            }
        }
    }
}

impl std::error::Error for self::Error {}
