use crate::model::todo::{CreateTodoRequest, NewTodo};
use crate::repository::todo::repository::TodoRepository;
use actix_web::{error, post, web, Error, HttpResponse};
use serde_json::json;

#[post("/todos")]
pub async fn post(todo_request: web::Json<CreateTodoRequest>) -> Result<HttpResponse, Error> {
    let repository = TodoRepository::new();
    let result = repository.insert(NewTodo {
        title: todo_request.title.clone(),
        memo: todo_request.memo.clone(),
        done: todo_request.done,
    });

    match result {
        Ok(id) => Ok(HttpResponse::Created().json(json!({ "id": id }))),
        Err(e) => Err(error::ErrorBadRequest(e)),
    }
}
