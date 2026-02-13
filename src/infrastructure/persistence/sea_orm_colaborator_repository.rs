use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::domain::{
    entities::company_colaborator::{CompanyColaborator, NewColaborator},
    exceptions::RepositoryError,
    repositories::colaborator_repository_interface::ColaboratorRepository,
};

pub struct SeaOrmColaboratorRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmColaboratorRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl ColaboratorRepository for SeaOrmColaboratorRepository {
    async fn save(
        &self,
        colaborator: &NewColaborator,
    ) -> Result<CompanyColaborator, RepositoryError> {
        Ok(CompanyColaborator {
            id: Some(2),
            company_id: 2,
            user_id: 2,
            document: String::from("value"),
            badge: String::from("value"),
        })
    }

    async fn all(&self) -> Result<Vec<CompanyColaborator>, RepositoryError> {
        Ok(vec![CompanyColaborator {
            id: Some(2),
            company_id: 2,
            user_id: 2,
            document: String::from("value"),
            badge: String::from("value"),
        }])
    }

    async fn find_by_id(&self, colaborator_id: i32) -> Result<CompanyColaborator, RepositoryError> {
        Ok(CompanyColaborator {
            id: Some(2),
            company_id: 2,
            user_id: 2,
            document: String::from("value"),
            badge: String::from("value"),
        })
    }
}
