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
