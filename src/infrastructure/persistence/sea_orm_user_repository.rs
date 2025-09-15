use std::{str::FromStr, sync::Arc};

use crate::{
    domain::{
        entities::user::{NewUser, User},
        exceptions::RepositoryError,
        repositories::user_repository_interface::UserRepository,
    },
    infrastructure::mappers::user_mapper::UserMapper,
};
use core_server::RoleEnum;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
    QueryFilter, QuerySelect,
};

use crate::infrastructure::entities::{role, user};

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
        let model = user::ActiveModel {
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
        let user = user::Entity::find_by_id(id)
            .find_with_related(role::Entity)
            .all(&*self.conn)
            .await?;

        match user.first() {
            Some(data) => {
                let (user, roles) = data;
                let roles: Vec<RoleEnum> = roles
                    .into_iter()
                    .map(|r| RoleEnum::from_str(&r.name).unwrap())
                    .collect();

                Ok(UserMapper::with_roles(User::from(user), roles))
            }
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn find_by_email(&self, email: String) -> Result<User, RepositoryError> {
        match user::Entity::find()
            .filter(user::Column::Email.eq(email))
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
