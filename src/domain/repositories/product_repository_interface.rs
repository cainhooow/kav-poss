use crate::domain::{
    entities::product::product::{NewProduct, Product},
    exceptions::RepositoryError,
};

#[async_trait::async_trait]
pub trait ProductRepository: Send + Sync {
    async fn all(&self) -> Result<Vec<Product>, RepositoryError>;
    async fn save(&self, product: &NewProduct) -> Result<Product, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Product, RepositoryError>;
}
