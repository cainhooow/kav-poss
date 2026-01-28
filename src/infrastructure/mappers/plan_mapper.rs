use crate::{domain::entities::plan::Plan, infrastructure::entities::plan::Model as PlanModel};

impl From<PlanModel> for Plan {
    fn from(value: PlanModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            description: value.description,
            price: value.price,
        }
    }
}
