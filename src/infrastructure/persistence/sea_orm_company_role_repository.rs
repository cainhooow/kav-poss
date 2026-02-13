use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::domain::{
    entities::company_role::{CompanyRole, NewCompanyRole},
    exceptions::RepositoryError,
    repositories::company_role_repository_interface::CompanyRoleRepository,
};

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
        Ok(CompanyRole {
            id: Some(2),
            name: String::from("value"),
            description: Some(String::from("value")),
            company_id: 10,
        })
    }

    async fn all(&self) -> Result<Vec<CompanyRole>, RepositoryError> {
        Ok(vec![CompanyRole {
            id: Some(2),
            name: String::from("value"),
            description: Some(String::from("value")),
            company_id: 10,
        }])
    }

    async fn find_by_id(&self, role_id: i32) -> Result<CompanyRole, RepositoryError> {
        Ok(CompanyRole {
            id: Some(2),
            name: String::from("value"),
            description: Some(String::from("value")),
            company_id: 10,
        })
    }
}
