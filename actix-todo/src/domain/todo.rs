use chrono::NaiveDateTime;

use crate::error::types::Error;

pub struct Todo {
    id: i32,
    title: String,
    memo: Option<String>,
    done: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Todo {
    pub fn insert() -> Result<(), Error> {
        Ok(())
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
