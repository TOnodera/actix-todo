use crate::error::types::Error;
use crate::repository::diesel::schema::todos;
use diesel::r2d2::ConnectionManager;
use diesel::{prelude::*, PgConnection, QueryResult};
use r2d2::PooledConnection;

use crate::model::todo::{NewTodo, Todo};
pub struct TodoRepository {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl TodoRepository {
    pub fn new(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        TodoRepository { connection }
    }
    pub fn insert(&self, todo: NewTodo) -> Result<i32, Error> {
        let result = diesel::insert_into(todos::table)
            .values(&todo)
            .get_result::<Todo>(&self.connection) as QueryResult<Todo>;
        match result {
            Ok(row) => Ok(row.id),
            Err(_) => Err(Error::DatabaseRuntimeError(
                "データの登録に失敗しました。".to_string(),
            )),
        }
    }
}
