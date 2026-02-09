use core_server::RoleEnum;
use garde::{Valid, Validate};
use serde::{Deserialize, Serialize};

use crate::domain::entities::plan::Plan;

#[derive(Serialize, Deserialize, Validate)]
pub struct PlanRequest {
    #[garde(ascii, length(min = 5))]
    pub name: String,
    #[garde(range(min = 0))]
    pub price: i32,
    #[garde(ascii, length(min = 5, max = 1000), required)]
    pub description: Option<String>,
    #[garde(required)]
    pub features: Option<Vec<RoleEnum>>,
}

#[derive(Serialize, Deserialize)]
pub struct PlanResource {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
}

impl From<&Plan> for PlanResource {
    fn from(value: &Plan) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            description: value.description.clone(),
            price: value.price.clone(),
        }
    }
}

impl From<Plan> for PlanResource {
    fn from(value: Plan) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            price: value.price,
        }
    }
}
