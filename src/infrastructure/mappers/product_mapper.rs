use crate::{
    domain::entities::product::product::Product,
    infrastructure::entities::product::Model as ProductModel,
};

impl From<ProductModel> for Product {
    fn from(value: ProductModel) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
            description: value.description,
        }
    }
}
