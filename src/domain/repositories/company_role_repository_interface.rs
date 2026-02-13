use salvo::async_trait;

use crate::domain::{
    entities::company_role::{CompanyRole, NewCompanyRole},
    exceptions::RepositoryError,
};

#[async_trait::async_trait]
pub trait CompanyRoleRepository: Send + Sync {
    async fn save(&self, role: NewCompanyRole) -> Result<CompanyRole, RepositoryError>;
    async fn all(&self) -> Result<Vec<CompanyRole>, RepositoryError>;
    async fn find_by_id(&self, role_id: i32) -> Result<CompanyRole, RepositoryError>;
}
