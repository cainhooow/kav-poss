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
}

impl From<&User> for UserResource {
    fn from(value: &User) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            email: value.email.clone(),
        }
    }
}

impl From<User> for UserResource {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}

impl UserResource {
    pub fn collection(items: Vec<User>) -> Vec<Self> {
        items.into_iter().map(UserResource::from).collect()
    }
}
