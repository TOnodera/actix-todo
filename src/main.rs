use actix_web::{App, HttpServer};
mod api;
mod consts;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(api::todo::get))
        .bind((
            consts::variables::SERVER_HOST,
            consts::variables::SERVER_PORT,
        ))?
        .run()
        .await
}
