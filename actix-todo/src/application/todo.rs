use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::{
    api::model::request::todo::RequestForCreate,
    domain::{repository::interface::Crud, service::todo::TodoDomainService, todo::NewTodoDomain},
    error::types::AppError,
    repository::{diesel::connection::Pool, todo::repository::TodoRepository},
};

pub struct TodoApplicationService;

impl TodoApplicationService {
    pub fn add_todo(
        state: web::Data<Pool>,
        request: web::Json<RequestForCreate>,
    ) -> Result<HttpResponse, AppError> {
        let conn = state.get()?;

        let repository = TodoRepository::new(conn);
        let result = TodoDomainService::add_todo(
            repository,
            NewTodoDomain {
                title: request.title.clone(),
                memo: request.memo.clone(),
                done: request.done,
            },
        );

        result.map(|id| HttpResponse::Created().json(json!({ "id": id })))
    }
}
