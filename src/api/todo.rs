use crate::repository;
use actix_web::{get, HttpResponse, Responder};
#[get("/todos")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok().body("hello")
}
