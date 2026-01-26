#[derive(Debug, Clone)]
pub struct Plan {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
}

pub struct PlanBuilder {
    name: String,
    description: Option<String>,
    price: i32,
}

#[derive(Clone, Debug)]
pub struct NewPlan {
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
}

impl PlanBuilder {
    pub fn new(name: impl Into<String>, price: i32) -> Self {
        Self {
            name: name.into(),
            description: None,
            price,
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
