use crate::domain::entities::plan::NewPlan;

pub struct PlanBuilder {
    name: String,
    description: Option<String>,
    price: i32,
}

impl PlanBuilder {
    pub fn new(name: impl Into<String>, price: i32) -> Self {
        Self {
            name: name.into(),
            description: None,
            price: price,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn price(mut self, price: i32) -> Self {
        self.price = price;
        self
    }

    pub fn build(self) -> NewPlan {
        NewPlan {
            name: self.name,
            description: self.description,
            price: self.price,
        }
    }
}
