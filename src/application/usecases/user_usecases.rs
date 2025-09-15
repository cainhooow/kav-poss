use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use core_server::RoleEnum;
use crypto::common::rand_core::OsRng;

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

pub struct CreateUserWithRolesUseCase<U: UserRepository, R: RoleRepository> {
    user_repository: U,
    role_repository: R,
}

impl<U: UserRepository, R: RoleRepository> CreateUserWithRolesUseCase<U, R> {
    pub fn new(user_repository: U, role_repository: R) -> Self {
        Self {
            user_repository,
            role_repository,
        }
    }

    pub async fn execute(
        &self,
        name: String,
        email: String,
        password: &String,
        roles: Vec<RoleEnum>,
    ) -> AppResult<User> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password = argon2.hash_password(password.as_bytes(), &salt)?;

        let user = UserBuilder::new(name, email, password.to_string()).build();
        match self.user_repository.save(&user).await {
            Ok(user) => {
                self.role_repository
                    .assign_roles_to_user(roles, user.id.unwrap())
                    .await?;
                Ok(user)
            }
            Err(err) => AppResult::Err(err.into()),
        }
    }
}
