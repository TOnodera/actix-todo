use crate::{
    domain::{
        repository::interface::Crud,
        todo::{NewTodoDomain, TodoDomain},
    },
    error::types::Error,
    repository::model::todo::RepositoryForCreate,
};

pub(crate) struct TodoDomainService;

impl TodoDomainService {
    pub fn add_todo(repository: impl Crud, todo: NewTodoDomain) -> Result<i32, Error> {
        let result = TodoDomain::insert(
            repository,
            RepositoryForCreate {
                title: todo.title.clone(),
                memo: todo.memo.clone(),
                done: todo.done,
            },
        );

        result
            .map(|id| id)
            .map_err(|e| Error::DatabaseRuntimeError(e.to_string()))
    }
}
