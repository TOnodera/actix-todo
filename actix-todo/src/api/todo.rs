use crate::model::todo::{NewTodo, TodoRequest};
use crate::repository::todo::repository::TodoRepository;
use actix_web::{error, get, post, web, Error, HttpResponse, Responder};
use serde_json::json;

#[get("/todos")]
pub async fn gets() -> Result<HttpResponse, Error> {
    let repository = TodoRepository::new();
    let result = repository.gets();
    match result {
        Ok(todos) => Ok(HttpResponse::Ok().json(todos)),
        Err(e) => Err(error::ErrorBadRequest(e)),
    }
}

#[post("/todos")]
pub async fn post(todo_request: web::Json<TodoRequest>) -> impl Responder {
    let repository = TodoRepository::new();
    let result = repository.insert(NewTodo {
        title: todo_request.title.clone(),
        memo: todo_request.memo.clone(),
        done: todo_request.done,
    });
    match result {
        Ok(id) => HttpResponse::Created().json(json!({ "id": id })),
        Err(e) => HttpResponse::BadRequest().json(json!({ "message": e.to_string()})),
    }
}
