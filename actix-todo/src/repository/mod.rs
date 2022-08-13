use actix_web::web;

pub mod diesel;
pub mod model;
pub mod todo;
pub fn connection(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(
        self::diesel::connection::establish_connection(),
    ));
}
