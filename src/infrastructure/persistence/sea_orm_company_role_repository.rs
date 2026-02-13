use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::domain::{
    entities::company_role::{CompanyRole, NewCompanyRole},
    exceptions::RepositoryError,
    repositories::company_role_repository_interface::CompanyRoleRepository,
};

use crate::infrastructure::entities::company_role;

pub struct SeaOrmCompanyRoleRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmCompanyRoleRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl CompanyRoleRepository for SeaOrmCompanyRoleRepository {
    async fn save(&self, role: &NewCompanyRole) -> Result<CompanyRole, RepositoryError> {
        let model = company_role::ActiveModel {
            name: Set(role.name.clone()),
            description: Set(role.description.clone()),
            company_id: Set(role.company_id.clone()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(CompanyRole::from(data)),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn all(&self) -> Result<Vec<CompanyRole>, RepositoryError> {
        match company_role::Entity::find().all(&*self.conn).await {
            Ok(data) => Ok(data.into_iter().map(CompanyRole::from).collect()),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn find_by_id(&self, role_id: i32) -> Result<CompanyRole, RepositoryError> {
        match company_role::Entity::find_by_id(role_id)
            .one(&*self.conn)
            .await
        {
            Ok(data) => {
                if let Some(role) = data {
                    Ok(CompanyRole::from(role))
                } else {
                    Err(RepositoryError::NotFound)
                }
            }
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
