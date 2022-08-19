use actix_web::HttpResponse;
use r2d2;
use serde_json::json;
use std::fmt::Display;
#[derive(Debug)]
pub enum AppError<'a> {
    ConnectionError(&'a str),
    DatabaseRuntimeError(&'a str),
    VarError(&'a str),
    NotFoundError(&'a str),
}

impl<'a> Display for AppError<'a> {
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
impl<'a> actix_web::error::ResponseError for AppError<'a> {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            _ => HttpResponse::InternalServerError()
                .json(json!({"message": "内部エラーが発生しました。"})),
        }
    }
}

// r2d2エラーの変換
impl<'a> From<r2d2::Error> for AppError<'a> {
    fn from(_: r2d2::Error) -> Self {
        AppError::ConnectionError("接続エラーが発生しました。")
    }
}
