use crate::{
    domain::{
        entities::company::{Company, NewCompany},
        exceptions::RepositoryError,
        repositories::company_repository_interface::CompanyRepository,
    },
    infrastructure::entities::company::{self as CompanyModel, Entity as CompanyEntity},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use std::sync::Arc;

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
        let model = CompanyModel::ActiveModel {
            name: Set(company.name.clone()),
            user_id: Set(company.user_id.clone()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(Company::from(data)),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn find_by_id(&self, id: i32) -> Result<Company, RepositoryError> {
        match CompanyEntity::find_by_id(id).one(&*self.conn).await {
            Ok(data) => {
                if let Some(company) = data {
                    Ok(Company::from(company))
                } else {
                    Err(RepositoryError::NotFound)
                }
            }
            Err(err) => Err(RepositoryError::Generic(err.to_string()))
        }
    }

    async fn find_by_user_id(&self, user_id: i32) -> Result<Company, RepositoryError> {
        Ok(Company {
            id: Some(21),
            name: String::from("Company"),
            user_id,
        })
    }
}
