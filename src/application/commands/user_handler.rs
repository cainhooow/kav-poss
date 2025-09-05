use crate::{
    application::exceptions::AppResult,
    domain::{
        entities::user::{User, UserBuilder},
        repositories::user_repository_interface::UserRepository,
    },
};

pub struct CreateUserUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, name: String, email: String, password: String) -> AppResult<User> {
        let user = UserBuilder::new(name, email, password).build();
        Ok(self.repository.save(&user).await?)
    }
}
