use actix_web::{error, web, HttpResponse};
use serde_json::json;

use crate::{
    api::model::request::todo::RequestForCreate,
    domain::{repository::interface::Crud, todo::TodoDomain},
    repository::{
        diesel::connection::Pool, model::todo::RepositoryForCreate,
        todo::repository::TodoRepository,
    },
};

pub struct ApplicationService;

impl ApplicationService {
    pub fn add_todo(
        state: web::Data<Pool>,
        request: web::Json<RequestForCreate>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let conn = state.get().unwrap();
        let repository = TodoRepository::new(conn);
        let result = TodoDomain::insert(
            repository,
            RepositoryForCreate {
                title: request.title.clone(),
                memo: request.memo.clone(),
                done: request.done,
            },
        );

        result
            .map(|id| HttpResponse::Created().json(json!({ "id": id })))
            .map_err(|e| error::ErrorBadRequest(e))
    }
}
