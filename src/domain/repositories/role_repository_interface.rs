use crate::domain::{entities::{role::{NewRole, Role, RolesEnum}, user::User}, exceptions::RepositoryError};


#[async_trait::async_trait]
pub trait RoleRepository: Send + Sync {
    async fn save(&self, role: &NewRole) -> Result<Role, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Role, RepositoryError>;
    async fn select_roles(&self, select: Vec<RolesEnum>) -> Result<Vec<Role>, RepositoryError>;
    async fn assign_roles_to_user(&self, role: Vec<RolesEnum>, user_id: i32) -> Result<(), RepositoryError>;
}
