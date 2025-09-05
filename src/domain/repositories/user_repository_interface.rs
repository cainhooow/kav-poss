use crate::domain::{entities::user::User, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<User, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<User, RepositoryError>;
    async fn find_by_email(&self, email: impl Into<String>) -> Result<User, RepositoryError>;
}
