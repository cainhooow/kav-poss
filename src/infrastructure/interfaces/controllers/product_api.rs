use salvo::prelude::*;
use std::sync::Arc;

use crate::{
    domain::entities::product::product::Product,
    infrastructure::interfaces::resources::{DataResponse, product_resource::ProductResource},
};

#[handler]
async fn get_products(
    depot: &mut Depot,
    res: &mut Response,
) -> Result<Json<DataResponse<ProductResource>>, String> {
    Ok(Json(DataResponse::success(ProductResource {
        id: Some(1),
        name: "xxx".to_string(),
        description: None,
    })))
}

#[handler]
async fn get_product_by_id(
    depot: &mut Depot,
    res: &mut Response,
) -> Result<Json<DataResponse<ProductResource>>, String> {
    Ok(Json(DataResponse::success(ProductResource {
        id: Some(1),
        name: "xxx".to_string(),
        description: None,
    })))
}

pub fn product_router() -> Router {
    Router::with_path("products")
        .get(get_products)
        .push(Router::with_path("/{id}").get(get_product_by_id))
}
