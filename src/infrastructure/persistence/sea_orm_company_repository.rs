use crate::{
    domain::{
        entities::company::{Company, NewCompany},
        exceptions::RepositoryError,
        repositories::company_repository_interface::CompanyRepository,
    },
    infrastructure::entities::{company, company_colaborator, company_role, role},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityLoaderTrait,
    EntityTrait, ModelTrait, QueryFilter,
};
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
        let model = company::ActiveModel {
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
        match company::Entity::load()
            .filter_by_id(id)
            .with(company_colaborator::Entity)
            .with((company_role::Entity, role::Entity))
            .one(&*self.conn)
            .await
        {
            Ok(data) => {
                if let Some(company) = data {
                    Ok(Company::from(company))
                } else {
                    Err(RepositoryError::NotFound)
                }
            }
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn find_by_user_id(&self, user_id: i32) -> Result<Company, RepositoryError> {
        match company::Entity::find()
            .filter(company::Column::UserId.eq(user_id))
            .one(&*self.conn)
            .await
        {
            Ok(data) => {
                if let Some(company) = data {
                    Ok(Company::from(company))
                } else {
                    Err(RepositoryError::NotFound)
                }
            }
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
