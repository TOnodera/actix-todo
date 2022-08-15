use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::repository::diesel::schema::todos;

#[derive(Insertable)]
#[table_name = "todos"]
pub struct RepositoryForCreate {
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
