use chrono::NaiveDateTime;

use crate::{error::types::Error, repository::model::todo::RepositoryForCreate};

use super::repository::interface::Crud;

pub struct TodoDomain {
    id: i32,
    title: String,
    memo: Option<String>,
    done: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl TodoDomain {
    pub fn insert(repository: impl Crud, todo: RepositoryForCreate) -> Result<i32, Error> {
        Ok(repository.insert(todo)?)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn 登録できる() {}
    fn 更新できる() {}
    fn 削除できる() {}
    fn 一件のデータを取得できる() {}
    fn 複数件のデータを取得できる() {}
}
