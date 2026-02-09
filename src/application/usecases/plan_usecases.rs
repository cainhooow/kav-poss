use core_server::RoleEnum;

use crate::{
    application::exceptions::{AppError, AppResult},
    domain::{
        entities::plan::{Plan, PlanBuilder},
        repositories::{
            plan_repository_interface::PlanRepository, role_repository_interface::RoleRepository,
        },
    },
};

pub struct CreatePlanUseCase<R: PlanRepository, P: RoleRepository> {
    repository: R,
    role_repository: P,
}

impl<R: PlanRepository, P: RoleRepository> CreatePlanUseCase<R, P> {
    pub fn new(repo: R, role_repository: P) -> Self {
        Self {
            repository: repo,
            role_repository,
        }
    }

    pub async fn execute(
        &self,
        name: String,
        price: i32,
        description: Option<String>,
        flags: Option<Vec<RoleEnum>>,
    ) -> AppResult<Plan> {
        let plan = PlanBuilder::new(name, price)
            .description(description)
            .build();

        match self.repository.save(&plan).await {
            Ok(data) => {
                if let Some(flags) = flags {
                    self.role_repository
                        .assign_flags_to_plan(flags, data.id.unwrap())
                        .await?;
                    Ok(data)
                } else {
                    Ok(data)
                }
            }
            Err(err) => AppResult::Err(err.into()),
        }
    }
}

// pub struct CreatePlanFeatureUseCase<P: PlanRepository, R: RoleRepository> {
//     repository: P,
//     role_repository: R,
// }

// impl<P: PlanRepository, R: RoleRepository> CreatePlanFeatureUseCase<P, R> {
//     pub fn new(plan_repository: P, role_repository: R) -> Self {
//         Self {
//             repository: plan_repository,
//             role_repository,
//         }
//     }

//     pub async fn execute(&self, plan_id: i32, flags: Vec<RoleEnum>) -> AppResult<Vec<String>> {
//         Ok(vec![String::from("")])
//     }
// }
