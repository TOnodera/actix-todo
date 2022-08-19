use crate::domain::repository::interface::Crud;
use crate::error::types::AppError;
use crate::repository::diesel::schema::todos;
use crate::repository::model::todo::ModelForCreate;
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

    fn insert(&self, todo: ModelForCreate) -> Result<i32, AppError> {
        let result = diesel::insert_into(todos::table)
            .values(&todo)
            .get_result::<Todo>(&self.connection) as QueryResult<Todo>;
        match result {
            Ok(row) => Ok(row.id),
            Err(_) => Err(AppError::DatabaseRuntimeError(
                "データの登録に失敗しました。",
            )),
        }
    }

    fn gets(&self, offset: i32, limit: i32) -> Result<Vec<Todo>, AppError> {
        use crate::repository::diesel::schema::todos::dsl;

        let results = dsl::todos
            .offset(i64::from(offset))
            .limit(i64::from(limit))
            .load::<Todo>(&self.connection);
        match results {
            Ok(rows) => Ok(rows),
            Err(e) => Err(AppError::DatabaseRuntimeError(
                "Todoテーブルのデータの取得に失敗しました。",
            )),
        }
    }

    /*
    fn get(conn: &PgConnection, id: i32) -> Result<Todo, Error> {
        use crate::repository::diesel::schema::todos::dsl;

        let results = dsl::todos
            .filter(dsl::id.eq(id))
            .limit(1)
            .load::<Todo>(conn);
        let row = match results {
            Ok(rows) => {
                if rows.len() == 0 {
                    return Err(Error::NotFoundError(format!(
                        "id={}のデータは存在しません。",
                        id
                    )));
                }
                rows[0].clone()
            }
            Err(e) => {
                return Err(Error::DatabaseRuntimeError(
                    format!("Todoテーブルのデータの取得に失敗しました。: {}", e).to_string(),
                ))
            }
        };
        Ok(row)
    }

    fn update(conn: &PgConnection, request: UpdateTodoRequest) -> Result<(), Error> {
        use crate::repository::diesel::schema::todos::dsl;
        let result = dsl::todos
            .filter(dsl::id.eq(request.id))
            .first::<Todo>(conn);
        let registered = match result {
            Ok(todo) => todo,
            Err(_) => {
                return Err(Error::DatabaseRuntimeError(
                    "更新に失敗しました。".to_string(),
                ));
            }
        };
        let result = diesel::update(&registered)
            .set((
                dsl::title.eq(request.title),
                dsl::memo.eq(request.memo),
                dsl::done.eq(request.done),
            ))
            .get_result::<Todo>(conn);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::DatabaseRuntimeError(
                "更新に失敗しました。".to_string(),
            )),
        }
    }

    fn delete(conn: &PgConnection, id: i32) -> Result<(), Error> {
        use crate::repository::diesel::schema::todos::dsl;
        let result = diesel::delete(dsl::todos.filter(dsl::id.eq(id))).execute(conn);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::DatabaseRuntimeError(e.to_string())),
        }
    }
    */
}
