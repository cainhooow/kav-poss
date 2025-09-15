use crate::domain::exceptions::RepositoryError;
use argon2::password_hash::Error as ArgonError;
use jsonwebtoken::errors::{Error as JWTError, ErrorKind as JWTErrorKind};
use std::{
    fmt::format,
    num::{IntErrorKind, ParseIntError},
};
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
        println!("ParseInt error: {}", value);
        let error_kind = value.kind();
        match error_kind {
            IntErrorKind::InvalidDigit => {
                AppError::Unexpected(format!("Provided value is invalid int(i32, i64)"))
            }
            _ => AppError::Unexpected(format!("ParseInt error")),
        }
    }
}

impl From<ArgonError> for AppError {
    fn from(err: ArgonError) -> Self {
        println!("Argo2 exception: {}", err);
        match err {
            _ => AppError::Unexpected(format!(
                "Argon2 Error: failed to hash password. Err: {}",
                err.to_string()
            )),
        }
    }
}

impl From<JWTError> for AppError {
    fn from(error: JWTError) -> Self {
        println!("JWT exception: {} ", error);
        let error_kind = error.kind();

        match error_kind {
            JWTErrorKind::InvalidToken => {
                AppError::Unexpected(format!("Invalid token: {}", error.to_string()))
            }
            JWTErrorKind::Json(msg) => {
                AppError::Unexpected(format!("Invalid token: {}", msg.to_string()))
            }
            _ => AppError::Unexpected(format!("")),
        }
    }
}
