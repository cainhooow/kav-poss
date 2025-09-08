use crate::domain::{
    entities::user::{NewUser, User},
    exceptions::RepositoryError,
};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &NewUser) -> Result<User, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<User, RepositoryError>;
    async fn find_by_email(&self, email: String) -> Result<User, RepositoryError>;
}
