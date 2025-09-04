use serde::{Deserialize, Serialize};
use crate::domain::entities::product::product::Product;
use rust_decimal::prelude::ToPrimitive;

#[derive(Serialize, Deserialize)]
pub struct ProductResource {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub sku: String
}

impl From<&Product> for ProductResource {
    fn from(value: &Product) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            description: value.description.clone(),
            price: value.price.to_f64().unwrap(),
            sku: value.sku.clone()
        }
    }
}

impl ProductResource {
    pub fn collection(items: Vec<&Product>) -> Vec<Self> {
        items.into_iter().map(ProductResource::from).collect()
    }
}
