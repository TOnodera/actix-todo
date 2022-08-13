use actix_web::{error, web, HttpResponse};
use serde_json::json;

use crate::{
    api::model::request::todo::RequestForCreate,
    domain::{repository::interface::Crud, service::todo::TodoDomainService, todo::NewTodoDomain},
    repository::{diesel::connection::Pool, todo::repository::TodoRepository},
};

pub struct TodoApplicationService;

impl TodoApplicationService {
    pub fn add_todo(
        state: web::Data<Pool>,
        request: web::Json<RequestForCreate>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let result = state.get();
        let conn = match result {
            Ok(conn) => conn,
            Err(e) => {
                return Err(error::ErrorInternalServerError(e));
            }
        };

        let repository = TodoRepository::new(conn);
        let result = TodoDomainService::add_todo(
            repository,
            NewTodoDomain {
                title: request.title.clone(),
                memo: request.memo.clone(),
                done: request.done,
            },
        );

        result
            .map(|id| HttpResponse::Created().json(json!({ "id": id })))
            .map_err(|e| error::ErrorInternalServerError(e))
    }
}
