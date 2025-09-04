#[derive(Debug, Clone)]
pub struct Product {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
}

pub struct ProductBuilder {
    name: String,
    description: Option<String>,
}

#[derive(Clone)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
}

impl ProductBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> NewProduct {
        NewProduct {
            name: self.name,
            description: self.description,
        }
    }
}
