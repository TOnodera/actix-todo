use crate::error::types::Error;
use crate::repository::diesel::connection;
use crate::repository::diesel::schema::todos;
use crate::utils;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TodoRequest {
    pub title: String,
    pub memo: Option<String>,
    pub done: bool,
}

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
    pub fn insert(todo: NewTodo) -> Result<i32, Error> {
        let url = utils::EnvFile::database_url()?;
        let conn = connection::get_connection(url)?;
        let result = diesel::insert_into(todos::table)
            .values(&todo)
            .get_result::<Todo>(&conn) as QueryResult<Todo>;
        match result {
            Ok(row) => Ok(row.id),
            Err(_) => Err(Error::DatabaseRuntimeError(
                "データの登録に失敗しました。".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn データの登録ができる() {
        let new_todo = NewTodo {
            title: "title".to_string(),
            memo: Some("memo".to_string()),
            done: false,
        };
        assert!(Todo::insert(new_todo).is_ok());
    }
    #[test]
    fn データの登録に失敗した場合はエラーを出力する() {
        let new_todo = NewTodo {
            title: "aaaaaaaaaa".to_string().repeat(26), // 255文字までで260字登録しようとしてエラー
            memo: Some("memo".to_string()),
            done: false,
        };
        let res = Todo::insert(new_todo);
        assert!(res.is_err());
        res.err().and_then(|e| {
            Some({
                assert_eq!(
                    e.to_string(),
                    "DatabaseRuntime Error: データの登録に失敗しました。"
                );
            })
        });
    }
}
