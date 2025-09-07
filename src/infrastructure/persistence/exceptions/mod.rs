use sea_orm::DbErr;

use crate::domain::exceptions::RepositoryError;

impl From<DbErr> for RepositoryError {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::Conn(msg) => RepositoryError::Generic(msg.to_string()),
            DbErr::Query(msg) => RepositoryError::Generic(msg.to_string()),
            DbErr::Custom(msg) => RepositoryError::Generic(msg.to_string()),
            DbErr::RecordNotFound(msg) => RepositoryError::Generic(msg.to_string()),
            DbErr::RecordNotUpdated => RepositoryError::Generic(String::from("Record not updated")),
            _ => RepositoryError::Generic(String::from("Error as ocurred in database")),
        }
    }
}
