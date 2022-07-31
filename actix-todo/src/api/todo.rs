use crate::model::todo::{NewTodo, TodoRequest};
use crate::repository::todo::repository::TodoRepository;
use actix_web::{get, post, web, HttpResponse, Responder};
#[get("/todos")]
pub async fn get() -> impl Responder {
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
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::BadRequest(),
    }
}
