use core::panic;

use diesel::PgConnection;

use crate::error::types::Error;
use crate::repository::diesel::connection as diesel_connection;
use crate::utils;

use crate::model::todo::{NewTodo, Todo};
pub struct TodoRepository {
    connection: PgConnection,
}

impl TodoRepository {
    pub fn new() -> Self {
        let result = match utils::EnvFile::database_url() {
            Ok(url) => diesel_connection::get_connection(url),
            Err(e) => panic!("{}", e),
        };
        let connection = match result {
            Ok(connection) => connection,
            Err(e) => panic!("{}", e),
        };

        TodoRepository { connection }
    }
    pub fn insert(&self, todo: NewTodo) -> Result<i32, Error> {
        Todo::insert(&self.connection, todo)
    }
}
