use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, QueryFilter};

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
        let model = plan::ActiveModel {
            name: Set(plan.name.clone()),
            price: Set(plan.price.clone()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(Plan::from(data)),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn find_by_id(&self, plan_id: i32) -> Result<Plan, RepositoryError> {
        match plan::Entity::find_by_id(plan_id).one(&*self.conn).await {
            Ok(data) => {
                if let Some(plan) = data {
                    Ok(Plan::from(plan))
                } else {
                    Err(RepositoryError::NotFound)
                }
            }
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
