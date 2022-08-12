use crate::api::model::request::todo::RequestForCreate;
use crate::application::todo::ApplicationService;
use crate::repository::diesel::connection::Pool;
use actix_web::{post, web, Error, HttpResponse};

#[post("/todos")]
pub async fn post(
    request: web::Json<RequestForCreate>,
    state: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    ApplicationService::add_todo(state, request)
}
