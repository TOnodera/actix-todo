use crate::repository::diesel::schema::todos;

#[derive(Insertable)]
#[table_name = "todos"]
pub struct RepositoryForCreate {
    pub title: String,
    pub memo: Option<String>,
    pub done: bool,
}
