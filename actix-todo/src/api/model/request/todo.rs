use serde::Deserialize;

#[derive(Deserialize)]
pub struct Create {
    pub title: String,
    pub memo: Option<String>,
    pub done: bool,
}
