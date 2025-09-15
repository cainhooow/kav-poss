use core_server::RoleEnum;
use sea_orm::ModelTrait;

use crate::domain::entities::user::User;
use crate::infrastructure::entities::{role::Model as RoleModel, user::Model as UserModel};

pub struct UserMapper;

impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            email: value.email,
            password: value.password,
            roles: vec![],
        }
    }
}

impl From<&UserModel> for User {
    fn from(value: &UserModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name.clone(),
            email: value.email.clone(),
            password: value.password.clone(),
            roles: vec![],
        }
    }
}

impl UserMapper {
    pub fn with_roles(mut user: User, roles: Vec<RoleEnum>) -> User {
        user.roles = roles;
        user
    }
}
