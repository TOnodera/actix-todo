#[macro_use]
extern crate diesel;
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use model::todo::Todo;
use r2d2::Pool;

mod api;
mod config;
mod error;
mod model;
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
