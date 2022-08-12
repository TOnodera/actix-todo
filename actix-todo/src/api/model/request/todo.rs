use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestForCreate {
    pub title: String,
    pub memo: Option<String>,
    pub done: bool,
}
