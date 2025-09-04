use crate::{
    application::exceptions::AppResult,
    domain::{
        entities::product::product::{Product, ProductBuilder},
        repositories::product_repository_interface::ProductRepository,
    },
};

pub struct CreateProductUseCase<R: ProductRepository> {
    repository: R,
}

pub struct FindProductUseCase<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> CreateProductUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, name: String) -> AppResult<Product> {
        let product = ProductBuilder::new(name).build();
        let created_product = self.repository.save(&product).await?;
        Ok(created_product)
    }
}

impl<R: ProductRepository> FindProductUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(&self, id: i32) -> AppResult<Product> {
        let product = self.repository.find_by_id(id).await?;
        Ok(product)
    }
}
