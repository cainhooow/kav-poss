use salvo::{writing::Text, Depot, Request, Response, Writer};
use thiserror::Error;

use crate::{
    domain::exceptions::RepositoryError,
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Domain error: {0}")]
    Domain(String),

    #[error("Repository error: {0}")]
    Repository(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<RepositoryError> for AppError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound => AppError::Repository("Entity not found".into()),
            RepositoryError::Generic(e) => AppError::Repository(e),
        }
    }
}

// #[async_trait::async_trait]
// impl Writer for AppError {
//     async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
//         println!("{:?} {:?} {:?}", depot, _req, res);
//         res.render(Text::Plain("I'm a error, hahaha!"));
//     }
// }