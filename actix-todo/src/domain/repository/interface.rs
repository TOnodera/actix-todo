use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;

use crate::{
    error::types::AppError,
    repository::model::todo::{ModelForCreate, Todo},
};

pub trait Crud {
    fn new(conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self;
    fn insert(&self, todo: ModelForCreate) -> Result<i32, AppError>;
    fn gets(&self, offset: i32, limit: i32) -> Result<Vec<Todo>, AppError>;
    // fn get(&self, id: i32) -> Result<Todo, AppError>;
}
