use crate::{
    domain::{
        repository::interface::Crud,
        todo::{NewTodoDomain, TodoDomain},
    },
    error::types::AppError,
    repository::model::todo::ModelForCreate,
};

pub(crate) struct TodoDomainService;

impl TodoDomainService {
    pub fn add_todo(repository: impl Crud, todo: NewTodoDomain) -> Result<i32, AppError<'static>> {
        TodoDomain::insert(
            repository,
            ModelForCreate {
                title: todo.title.clone(),
                memo: todo.memo.clone(),
                done: todo.done,
            },
        )
    }
}
