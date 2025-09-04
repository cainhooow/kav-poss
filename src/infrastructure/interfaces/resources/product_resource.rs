use serde::Serialize;

use crate::domain::entities::product::product::Product;

#[derive(Serialize)]
pub struct ProductResource {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
}

impl From<&Product> for ProductResource {
    fn from(value: &Product) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            description: value.description.clone(),
        }
    }
}

impl ProductResource {
    pub fn collection(items: Vec<&Product>) -> Vec<Self> {
        items.into_iter().map(ProductResource::from).collect()
    }
}
