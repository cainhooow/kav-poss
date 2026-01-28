use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    domain::{
        entities::plan::{NewPlan, Plan},
        exceptions::RepositoryError,
        repositories::plan_repository_interface::PlanRepository,
    },
    infrastructure::entities::plan,
};

pub struct SeaOrmPlanRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmPlanRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl PlanRepository for SeaOrmPlanRepository {
    async fn save(&self, plan: &NewPlan) -> Result<Plan, RepositoryError> {
        Ok(Plan {
            id: Some(1),
            name: String::from(""),
            description: Some(String::from("")),
            price: 100,
        })
    }

    async fn find_by_id(&self, id: i32) -> Result<Plan, RepositoryError> {
        Ok(Plan {
            id: Some(1),
            name: String::from(""),
            description: Some(String::from("")),
            price: 100,
        })
    }
}
