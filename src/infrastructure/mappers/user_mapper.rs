use crate::domain::entities::user::User;
use crate::infrastructure::entities::user::Model as UserModel;

impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            email: value.email,
            password: value.password,
        }
    }
}
