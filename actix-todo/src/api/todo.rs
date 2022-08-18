use crate::api::model::request::todo::RequestForCreate;
use crate::api::model::response::AppResponse;
use crate::application::todo::TodoApplicationService;
use crate::repository::diesel::connection::Pool;
use actix_web::{post, web};

#[post("/todos")]
pub async fn post(request: web::Json<RequestForCreate>, state: web::Data<Pool>) -> AppResponse {
    TodoApplicationService::add_todo(state, request)
}
