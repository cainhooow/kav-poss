use crate::{
    application::exceptions::AppResult,
    domain::{entities::plan::Plan, repositories::plan_repository_interface::PlanRepository},
};

pub struct FindPlanByIdQuery<R: PlanRepository> {
    repository: R,
}

impl<R: PlanRepository> FindPlanByIdQuery<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, id: i32) -> AppResult<Plan> {
        let plan = self.repository.find_by_id(id).await?;
        Ok(plan)
    }
}
