use core::panic;

use diesel::PgConnection;

use crate::error::types::Error;
use crate::repository::diesel::connection as diesel_connection;
use crate::utils;

use crate::model::todo::{NewTodo, Todo, UpdateTodoRequest};
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
    pub fn gets(&self, offset: i64, limit: i64) -> Result<Vec<Todo>, Error> {
        Todo::gets(&self.connection, offset, limit)
    }
    pub fn get(&self, id: i32) -> Result<Todo, Error> {
        Todo::get(&self.connection, id)
    }
    pub fn update(&self, request: UpdateTodoRequest) -> Result<(), Error> {
        Todo::update(&self.connection, request)
    }
    pub fn delete(&self, id: i32) -> Result<(), Error> {
        Todo::delete(&self.connection, id)
    }
}
