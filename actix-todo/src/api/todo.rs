use crate::repository::diesel::connection::Pool;
use crate::repository::todo::repository::TodoRepository;
use crate::{api::model::request::todo::Create, model::todo::NewTodo};
use actix_web::{error, post, web, Error, HttpResponse};
use serde_json::json;

#[post("/todos")]
pub async fn post(
    request: web::Json<Create>,
    state: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let conn = state.get().unwrap();
    let repository = TodoRepository::new(conn);
    let result = repository.insert(NewTodo {
        title: request.title.clone(),
        memo: request.memo.clone(),
        done: request.done,
    });

    match result {
        Ok(id) => Ok(HttpResponse::Created().json(json!({ "id": id }))),
        Err(e) => Err(error::ErrorBadRequest(e)),
    }
}
