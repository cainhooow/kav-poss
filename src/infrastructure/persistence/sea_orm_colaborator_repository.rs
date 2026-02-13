use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::domain::{
    entities::company_colaborator::{CompanyColaborator, NewColaborator},
    exceptions::RepositoryError,
    repositories::colaborator_repository_interface::ColaboratorRepository,
};

use crate::infrastructure::entities::company_colaborator;

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
        let model = company_colaborator::ActiveModel {
            document: Set(colaborator.document.clone()),
            badge: Set(colaborator.badge.clone()),
            user_id: Set(colaborator.user_id.clone()),
            company_id: Set(colaborator.company_id.clone()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(CompanyColaborator::from(data)),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn all(&self) -> Result<Vec<CompanyColaborator>, RepositoryError> {
        match company_colaborator::Entity::find().all(&*self.conn).await {
            Ok(data) => Ok(data.into_iter().map(CompanyColaborator::from).collect()),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn find_by_id(&self, colaborator_id: i32) -> Result<CompanyColaborator, RepositoryError> {
        match company_colaborator::Entity::find_by_id(colaborator_id)
            .one(&*self.conn)
            .await
        {
            Ok(data) => {
                if let Some(colaborator) = data {
                    Ok(CompanyColaborator::from(colaborator))
                } else {
                    Err(RepositoryError::NotFound)
                }
            }
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
