use std::sync::Arc;

use crate::domain::{
    entities::user::{NewUser, User},
    exceptions::RepositoryError,
    repositories::user_repository_interface::UserRepository,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::infrastructure::entities::user::{
    self as UserModel, Column as UserCol, Entity as UserEntity,
};

pub struct SeaOrmUserRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmUserRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn save(&self, user: &NewUser) -> Result<User, RepositoryError> {
        let model = UserModel::ActiveModel {
            name: Set(user.name.clone()),
            email: Set(user.email.clone()),
            password: Set(user.password.clone()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(User::from(data)),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn find_by_id(&self, id: i32) -> Result<User, RepositoryError> {
        match UserEntity::find_by_id(id).one(&*self.conn).await {
            Ok(data) => {
                if let Some(user) = data {
                    Ok(User::from(user))
                } else {
                    Err(RepositoryError::NotFound)
                }
            }
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn find_by_email(&self, email: String) -> Result<User, RepositoryError> {
        match UserEntity::find()
            .filter(UserCol::Email.eq(email))
            .one(&*self.conn)
            .await
        {
            Ok(data) => {
                if let Some(user) = data {
                    Ok(User::from(user))
                } else {
                    Err(RepositoryError::NotFound)
                }
            }
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
