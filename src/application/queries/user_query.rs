use crate::{
    application::exceptions::AppResult,
    domain::{entities::user::User, repositories::user_repository_interface::UserRepository},
};

pub struct FindUserByEmailQuery<R: UserRepository> {
    repository: R,
}

pub struct FindUserByIdQuery<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> FindUserByEmailQuery<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, email: impl Into<String>) -> AppResult<User> {
        let user = self.repository.find_by_email(email.into()).await?;
        Ok(user)
    }
}

impl<R: UserRepository> FindUserByIdQuery<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, id: i32) -> AppResult<User> {
        let user = self.repository.find_by_id(id).await?;
        Ok(user)
    }
}
