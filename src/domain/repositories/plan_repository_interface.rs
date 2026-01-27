use crate::domain::{entities::plan::{NewPlan, Plan}, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait PlanRepository: Send + Sync {
    async fn save(&self, plan: &NewPlan) -> Result<Plan, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Plan, RepositoryError>;
}
