use core_server::RoleEnum;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use std::{str::FromStr, sync::Arc};

use crate::domain::{
    entities::role::{NewRole, Role},
    exceptions::RepositoryError,
    repositories::role_repository_interface::RoleRepository,
};

use crate::infrastructure::entities::plan_feature_pivot;
use crate::infrastructure::entities::role;
use crate::infrastructure::entities::user_roles_pivot;

pub struct SeaOrmRoleRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmRoleRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl RoleRepository for SeaOrmRoleRepository {
    async fn save(&self, role: &NewRole) -> Result<Role, RepositoryError> {
        let model = role::ActiveModel {
            name: Set(role.name.clone()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(Role::from(data)),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn find_by_id(&self, id: i32) -> Result<Role, RepositoryError> {
        match role::Entity::find_by_id(id).one(&*self.conn).await {
            Ok(data) => {
                if let Some(role) = data {
                    Ok(Role::from(role))
                } else {
                    Err(RepositoryError::NotFound)
                }
            }
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn select_roles(&self, select: Vec<RoleEnum>) -> Result<Vec<Role>, RepositoryError> {
        let roles_names: Vec<String> = select
            .into_iter()
            .map(|r| RoleEnum::to_string(&r))
            .collect();

        match role::Entity::find()
            .filter(role::Column::Name.is_in(roles_names))
            .all(&*self.conn)
            .await
        {
            Ok(data) => Ok(data.into_iter().map(Role::from).collect()),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn assign_roles_to_user(
        &self,
        roles: Vec<RoleEnum>,
        user_id: i32,
    ) -> Result<(), RepositoryError> {
        let roles = self.select_roles(roles).await?;

        for role in roles {
            let model = user_roles_pivot::ActiveModel {
                role_id: Set(role.id),
                user_id: Set(user_id),
            };

            model.insert(&*self.conn).await?;
        }

        Ok(())
    }

    async fn assign_flags_to_plan(
        &self,
        flags: Vec<RoleEnum>,
        plan_id: i32,
    ) -> Result<(), RepositoryError> {
        let flags = self.select_roles(flags).await?;

        for flag in flags {
            let model = plan_feature_pivot::ActiveModel {
                feature_id: Set(flag.id),
                plan_id: Set(plan_id),
            };

            model.insert(&*self.conn).await?;
        }

        Ok(())
    }
}
