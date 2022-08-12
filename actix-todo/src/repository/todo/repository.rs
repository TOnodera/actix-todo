use core::panic;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{ManageConnection, PooledConnection};

use crate::error::types::Error;
use crate::repository::diesel::connection::{self as diesel_connection, Pool};
use crate::utils;

use crate::model::todo::{NewTodo, Todo};
pub struct TodoRepository {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl TodoRepository {
    pub fn new(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        TodoRepository { connection }
    }
    pub fn insert(&self, todo: NewTodo) -> Result<i32, Error> {
        Todo::insert(&self.connection, todo)
    }
}
