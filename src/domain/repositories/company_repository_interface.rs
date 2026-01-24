use crate::domain::{
    entities::company::{Company, NewCompany},
    exceptions::RepositoryError,
};

#[async_trait::async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn save(&self, company: &NewCompany) -> Result<Company, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Company, RepositoryError>;
    async fn find_by_user_id(&self, user_id: i32) -> Result<Company, RepositoryError>;
}
