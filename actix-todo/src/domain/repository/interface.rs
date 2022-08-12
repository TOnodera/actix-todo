use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;

use crate::{error::types::Error, repository::model::todo::RepositoryForCreate};

pub trait Crud {
    fn new(conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self;
    fn insert(&self, todo: RepositoryForCreate) -> Result<i32, Error>;
}
