use crate::domain::repository::interface::Crud;
use crate::error::types::Error;
use crate::repository::diesel::schema::todos;
use crate::repository::model::todo::RepositoryForCreate;
use diesel::r2d2::ConnectionManager;
use diesel::{prelude::*, PgConnection, QueryResult};
use r2d2::PooledConnection;

use crate::repository::model::todo::Todo;
pub struct TodoRepository {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl Crud for TodoRepository {
    fn new(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        TodoRepository { connection }
    }
    fn insert(&self, todo: RepositoryForCreate) -> Result<i32, Error> {
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
