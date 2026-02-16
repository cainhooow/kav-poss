use crate::domain::{
    entities::company_colaborator::{CompanyColaborator, NewColaborator},
    exceptions::RepositoryError,
};

#[async_trait::async_trait]
pub trait ColaboratorRepository: Send + Sync {
    async fn save(
        &self,
        colaborator: &NewColaborator,
    ) -> Result<CompanyColaborator, RepositoryError>;
    async fn all(&self, company_id: i32) -> Result<Vec<CompanyColaborator>, RepositoryError>;
    async fn find_by_id(&self, colaorator_id: i32) -> Result<CompanyColaborator, RepositoryError>;
    async fn find_by_user_id(
        &self,
        company_id: i32,
        user_id: i32,
    ) -> Result<CompanyColaborator, RepositoryError>;
}
