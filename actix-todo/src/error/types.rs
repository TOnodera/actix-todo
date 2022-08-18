use actix_web::{http::StatusCode, HttpResponse};
use r2d2;
use serde_json::json;
use std::fmt::Display;
#[derive(Debug)]
pub enum AppError {
    ConnectionError(String),
    DatabaseRuntimeError(String),
    VarError(String),
    NotFoundError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ConnectionError(e) => {
                write!(f, "Connection Error: {}", e)
            }
            AppError::DatabaseRuntimeError(e) => {
                write!(f, "DatabaseRuntime Error: {}", e)
            }
            AppError::VarError(e) => {
                write!(f, "VarError: {}", e)
            }
            AppError::NotFoundError(e) => {
                write!(f, "NotFoundError: {}", e)
            }
        }
    }
}

// AppErrorのResponseエラーとしての対応づけ
impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            _ => HttpResponse::InternalServerError()
                .json(json!({"message": "内部エラーが発生しました。"})),
        }
    }
}

// r2d2エラーの変換
impl From<r2d2::Error> for AppError {
    fn from(_: r2d2::Error) -> Self {
        AppError::ConnectionError("接続エラーが発生しました。".to_string())
    }
}
