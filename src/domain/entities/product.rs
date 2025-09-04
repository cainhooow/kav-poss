#[derive(Debug, Clone)]
pub struct Product {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub sku: String,
}

pub struct ProductBuilder {
    name: String,
    description: Option<String>,
    price: f64,
    sku: String,
}

#[derive(Clone, Debug)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub sku: String,
}

impl ProductBuilder {
    pub fn new(name: impl Into<String>, price: f64, sku: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            sku: sku.into(),
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

    pub fn price(mut self, price: f64) -> Self {
        self.price = price;
        self
    }

    pub fn sku(mut self, sku: impl Into<String>) -> Self {
        self.sku = sku.into();
        self
    }

    pub fn build(self) -> NewProduct {
        NewProduct {
            name: self.name,
            description: self.description,
            price: self.price,
            sku: self.sku,
        }
    }
}
