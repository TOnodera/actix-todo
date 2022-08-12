use crate::domain::repository::interface::Crud;
use crate::repository::diesel::connection::Pool;
use crate::repository::model::todo::RepositoryForCreate;
use crate::repository::todo::repository::TodoRepository;
use crate::{api::model::request::todo::RequestForCreate, domain::todo::TodoDomain};
use actix_web::{error, post, web, Error, HttpResponse};
use serde_json::json;

#[post("/todos")]
pub async fn post(
    request: web::Json<RequestForCreate>,
    state: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
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
