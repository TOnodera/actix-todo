use crate::model::todo::{NewTodo, TodoRequest};
use crate::repository::todo::repository::TodoRepository;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/todos")]
pub async fn gets() -> impl Responder {
    HttpResponse::Ok().body("hello")
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
