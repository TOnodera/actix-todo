use crate::error::types::Error;
use crate::repository::diesel::schema::todos;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, Clone, Associations)]
#[table_name = "todos"]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub memo: Option<String>,
    pub done: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
    pub memo: Option<String>,
    pub done: bool,
}

impl Todo {
    pub fn insert(
        conn: &PooledConnection<ConnectionManager<PgConnection>>,
        todo: NewTodo,
    ) -> Result<i32, Error> {
        let result = diesel::insert_into(todos::table)
            .values(&todo)
            .get_result::<Todo>(conn) as QueryResult<Todo>;
        match result {
            Ok(row) => Ok(row.id),
            Err(_) => Err(Error::DatabaseRuntimeError(
                "データの登録に失敗しました。".to_string(),
            )),
        }
    }
}
