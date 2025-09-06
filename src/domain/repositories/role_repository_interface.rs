use crate::domain::{entities::role::{NewRole, Role}, exceptions::RepositoryError};


#[async_trait::async_trait]
pub trait RoleRepository: Send + Sync {
    async fn save(&self, role: &NewRole) -> Result<Role, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Role, RepositoryError>;
    async fn select_roles(&self, select: Vec<NewRole>) -> Result<Vec<Role>, RepositoryError>;
}