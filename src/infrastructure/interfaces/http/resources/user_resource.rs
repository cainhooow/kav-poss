use core_server::RoleEnum;
use serde::{Deserialize, Serialize};

use crate::domain::entities::user::User;

#[derive(Deserialize)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResource {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub roles: Vec<RoleEnum>,
}

#[derive(Serialize, Deserialize)]
pub struct UserAuthResource {
    pub user: UserResource,
    #[serde(rename(serialize = "accessToken"))]
    pub access_token: String,
    #[serde(rename(serialize = "refreshToken"))]
    pub refresh_token: String,
}

impl From<&User> for UserResource {
    fn from(value: &User) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            email: value.email.clone(),
            roles: value.roles.clone(),
        }
    }
}

impl From<User> for UserResource {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            roles: value.roles.clone(),
        }
    }
}

impl UserResource {
    pub fn collection(items: Vec<User>) -> Vec<Self> {
        items.into_iter().map(UserResource::from).collect()
    }
}
