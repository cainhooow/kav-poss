use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityOrSelect,
    EntityTrait, QueryFilter,
};
use std::sync::Arc;

use crate::domain::{
    entities::role::{NewRole, Role},
    exceptions::RepositoryError,
    repositories::role_repository_interface::RoleRepository,
};

use crate::infrastructure::entities::role::{
    self as RoleModel, Column as RoleColumn, Entity as RoleEntity,
};

use crate::infrastructure::entities::user_roles_pivot::{
    self as RolePivotModel, Entity as PivotEntity,
};

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
        let model = RoleModel::ActiveModel {
            name: Set(role.name.clone()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(Role::from(data)),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn find_by_id(&self, id: i32) -> Result<Role, RepositoryError> {
        match RoleEntity::find_by_id(id).one(&*self.conn).await {
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

    async fn select_roles(&self, select: Vec<NewRole>) -> Result<Vec<Role>, RepositoryError> {
        let roles_names: Vec<String> = select.into_iter().map(|r| r.name).collect();

        match RoleEntity::find()
            .filter(RoleColumn::Name.is_in(roles_names))
            .all(&*self.conn)
            .await
        {
            Ok(data) => Ok(data.into_iter().map(Role::from).collect()),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
