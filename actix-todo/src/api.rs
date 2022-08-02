use actix_web::web;

pub mod todo;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(todo::gets).service(todo::post);
}
