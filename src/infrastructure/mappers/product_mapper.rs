use rust_decimal::prelude::ToPrimitive;

use crate::{
    domain::entities::product::Product,
    infrastructure::entities::product::Model as ProductModel,
};

impl From<ProductModel> for Product {
    fn from(value: ProductModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            description: value.description,
            price: value.price.to_f64().unwrap(),
            sku: value.sku,
        }
    }
}
