#[macro_use]
extern crate diesel;
use actix_web::{App, HttpServer};

mod api;
mod config;
mod error;
mod model;
mod repository;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(api::todo::gets).service(api::todo::post))
        .bind((config::consts::SERVER_HOST, config::consts::SERVER_PORT))?
        .run()
        .await
}
