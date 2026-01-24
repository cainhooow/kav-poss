use crate::{
    application::exceptions::AppResult,
    domain::{
        entities::product::{Product, ProductBuilder},
        repositories::product_repository_interface::ProductRepository,
    },
};

pub struct CreateProductUseCase<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> CreateProductUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repository: repo }
    }

    pub async fn execute(
        &self,
        name: String,
        price: i32,
        sku: String,
        description: Option<String>,
    ) -> AppResult<Product> {
        let product = ProductBuilder::new(name, price, sku)
            .description(description)
            .build();

        println!("product: {:?}", product);

        Ok(self.repository.save(&product).await?)
    }
}
