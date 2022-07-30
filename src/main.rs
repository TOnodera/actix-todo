use actix_web::{App, HttpServer};

mod api;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(api::todo::get))
        .bind((config::consts::SERVER_HOST, config::consts::SERVER_PORT))?
        .run()
        .await
}
