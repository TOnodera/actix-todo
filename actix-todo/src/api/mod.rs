use actix_web::web;

pub mod model;
pub mod todo;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(todo::post);
}
