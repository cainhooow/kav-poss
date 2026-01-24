use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::domain::{
    entities::company::{Company, NewCompany},
    exceptions::RepositoryError,
    repositories::company_repository::CompanyRepository,
};

pub struct SeaOrmCompanyRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmCompanyRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl CompanyRepository for SeaOrmCompanyRepository {
    async fn save(&self, company: &NewCompany) -> Result<Company, RepositoryError> {
        Ok(Company {
            id: Some(15),
            name: String::from("Company"),
            user_id: 1,
        })
    }

    async fn find_by_id(&self, id: i32) -> Result<Company, RepositoryError> {
        Ok(Company {
            id: Some(id),
            name: String::from("Company"),
            user_id: 15,
        })
    }

    async fn find_by_user_id(&self, user_id: i32) -> Result<Company, RepositoryError> {
        Ok(Company {
            id: Some(21),
            name: String::from("Company"),
            user_id,
        })
    }
}
