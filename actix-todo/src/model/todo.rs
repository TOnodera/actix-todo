use crate::error::types::Error;
use crate::repository::diesel::schema::todos;
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
    pub fn insert(conn: &PgConnection, todo: NewTodo) -> Result<i32, Error> {
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

    pub fn gets(conn: &PgConnection) -> Result<Vec<Todo>, Error> {
        use crate::repository::diesel::schema::todos::dsl::*;

        let results = todos.limit(5).load::<Todo>(conn);
        match results {
            Ok(rows) => Ok(rows),
            Err(e) => Err(Error::DatabaseRuntimeError(
                format!("Todoテーブルのデータの取得に失敗しました。: {}", e).to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{api::todo::gets, repository, utils};

    use super::*;
    #[test]
    fn データの登録ができる() {
        let new_todo = NewTodo {
            title: "title".to_string(),
            memo: Some("memo".to_string()),
            done: false,
        };
        let url = utils::EnvFile::database_url().unwrap();
        let conn = repository::diesel::connection::get_connection(url).unwrap();
        assert!(Todo::insert(&conn, new_todo).is_ok());
    }
    #[test]
    fn データの登録に失敗した場合はエラーを出力する() {
        let new_todo = NewTodo {
            title: "aaaaaaaaaa".to_string().repeat(26), // 255文字までで260字登録しようとしてエラー
            memo: Some("memo".to_string()),
            done: false,
        };
        let url = utils::EnvFile::database_url().unwrap();
        let conn = repository::diesel::connection::get_connection(url).unwrap();
        let res = Todo::insert(&conn, new_todo);
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

    #[test]
    fn データを取得できる() {
        let url = utils::EnvFile::database_url().unwrap();
        let conn = repository::diesel::connection::get_connection(url).unwrap();

        let results = Todo::gets(&conn);
        match results {
            Ok(rows) => {
                println!("{:#?}", rows);
            }
            Err(_) => {}
        }
    }
}
