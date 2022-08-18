use crate::error::types::AppError;
use actix_web::HttpResponse;

pub type AppResponse = Result<HttpResponse, AppError>;
