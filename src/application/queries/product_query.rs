use crate::{
    application::exceptions::AppResult,
    domain::{
        entities::product::product::Product,
        repositories::product_repository_interface::ProductRepository,
    },
};

pub struct FindProductByIdQuery<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> FindProductByIdQuery<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, id: i32) -> AppResult<Product> {
        let product = self.repository.find_by_id(id).await?;
        Ok(product)
    }
}

pub struct FindAllProductsQuery<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> FindAllProductsQuery<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self) -> AppResult<Vec<Product>> {
        let products = self.repository.all().await?;
        Ok(products)
    }
}
