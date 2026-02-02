use crate::{
    application::exceptions::AppResult,
    domain::{
        builders::plan_builder::PlanBuilder, entities::plan::Plan,
        repositories::plan_repository_interface::PlanRepository,
    },
};

pub struct CreatePlanUseCase<R: PlanRepository> {
    repository: R,
}

impl<R: PlanRepository> CreatePlanUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(
        &self,
        name: String,
        price: i32,
        description: Option<String>,
    ) -> AppResult<Plan> {
        let plan = PlanBuilder::new(name, price)
            .description(description)
            .build();

        Ok(self.repository.save(&plan).await?)
    }
}
