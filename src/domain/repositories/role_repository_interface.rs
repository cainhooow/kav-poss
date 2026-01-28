use core_server::RoleEnum;

use crate::domain::{
    entities::{
        role::{NewRole, Role},
        user::User,
    },
    exceptions::RepositoryError,
};

#[async_trait::async_trait]
pub trait RoleRepository: Send + Sync {
    async fn save(&self, role: &NewRole) -> Result<Role, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Role, RepositoryError>;
    async fn select_roles(&self, select: Vec<RoleEnum>) -> Result<Vec<Role>, RepositoryError>;
    async fn assign_roles_to_user(
        &self,
        roles: Vec<RoleEnum>,
        user_id: i32,
    ) -> Result<(), RepositoryError>;
    async fn assign_flags_to_plan(
        &self,
        flags: Vec<RoleEnum>,
        plan_id: i32,
    ) -> Result<(), RepositoryError>;
}
