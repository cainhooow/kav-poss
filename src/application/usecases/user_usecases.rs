use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::Salt};

use crate::{
    application::exceptions::AppResult,
    domain::{
        builders::user_builder::UserBuilder,
        entities::user::User,
        repositories::{
            role_repository_interface::RoleRepository, user_repository_interface::UserRepository,
        },
    },
};

pub struct CreateUserUseCase<R: UserRepository> {
    repository: R,
}

pub struct CreateUserWithRolesUseCase<U: UserRepository, R: RoleRepository> {
    user_repository: U,
    role_repository: R,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, name: String, email: String, password: &str) -> AppResult<User> {
        let argon2 = Argon2::default();
        let salt: Salt = password.try_into().unwrap();
        let password = argon2.hash_password(password.as_bytes(), salt)?;

        let user = UserBuilder::new(name, email, password.to_string()).build();
        Ok(self.repository.save(&user).await?)
    }
}

impl<U: UserRepository, R: RoleRepository> CreateUserWithRolesUseCase<U, R> {
    pub fn new(user_repository: U, role_repository: R) -> Self {
        Self {
            user_repository,
            role_repository,
        }
    }

    pub async fn execute(&self, name: String, email: String, password: &str) -> AppResult<User> {
        let argon2 = Argon2::default();
        let salt: Salt = password.try_into().unwrap();
        let password = argon2.hash_password(password.as_bytes(), salt)?;

        let user = UserBuilder::new(name, email, password.to_string()).build();
        Ok(self.user_repository.save(&user).await?)
    }
}
