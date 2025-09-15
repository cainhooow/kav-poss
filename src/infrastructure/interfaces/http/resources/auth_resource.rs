use super::user_resource::UserResource;
use crate::domain::entities::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResource {
    pub user: UserResource,
    #[serde(rename(serialize = "accessToken"))]
    pub access_token: String,
    #[serde(rename(serialize = "refreshToken"))]
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    #[serde(rename(deserialize = "refresh_token"))]
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct AuthRefreshResource {
    #[serde(rename(serialize="accessToken"))]
    pub access_token: String,
    #[serde(rename(serialize="refreshToken"))]
    pub refresh_token: String,
}

impl AuthResource {
    pub fn new(user: User, access_token: String, refresh_token: String) -> Self {
        Self {
            user: user.into(),
            access_token,
            refresh_token,
        }
    }
}
