use crate::domain::exceptions::RepositoryError;
use std::num::{IntErrorKind, ParseIntError};
use thiserror::Error;

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

impl From<ParseIntError> for AppError {
    fn from(value: ParseIntError) -> Self {
        let error_kind = value.kind();
        match error_kind {
            IntErrorKind::InvalidDigit => {
                AppError::Unexpected(format!("Provided value is invalid int(i32, i64)"))
            }
            _ => AppError::Unexpected(format!("ParseInt error")),
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        match err {
            _ => AppError::Unexpected(format!("Argon2 Error. failed to hash password")),
        }
    }
}
