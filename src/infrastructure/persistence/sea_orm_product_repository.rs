use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    domain::{
        entities::product::product::{NewProduct, Product},
        exceptions::RepositoryError,
        repositories::product_repository_interface::ProductRepository,
    },
    infrastructure::entities::product::{self as ProductModel, Entity as ProductEntity},
};

pub struct SeaOrmProductRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmProductRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl ProductRepository for SeaOrmProductRepository {
    async fn all(&self) -> Result<Vec<Product>, RepositoryError> {
        match ProductEntity::find().all(&*self.conn).await {
            Ok(data) => Ok(data.into_iter().map(Product::from).collect()),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn save(&self, product: &NewProduct) -> Result<Product, RepositoryError> {
        let model = ProductModel::ActiveModel {
            name: Set(product.name.clone()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(Product::from(data)),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn find_by_id(&self, id: i32) -> Result<Product, RepositoryError> {
        match ProductEntity::find_by_id(id).one(&*self.conn).await {
            Ok(data) => {
                if let Some(product) = data {
                    Ok(Product::from(product))
                } else {
                    Err(RepositoryError::NotFound)
                }
            }
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
