#[macro_use]
extern crate diesel;
use actix_web::{App, HttpServer};

mod api;
mod application;
mod config;
mod domain;
mod error;
mod repository;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(api::routes)
            .configure(repository::connection)
    })
    .bind((config::consts::SERVER_HOST, config::consts::SERVER_PORT))?
    .run()
    .await
}
