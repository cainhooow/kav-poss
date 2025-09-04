use salvo::prelude::*;
use std::sync::Arc;

use crate::{
    application::{
        commands::product_handler::CreateProductUseCase,
        exceptions::AppResult,
        queries::product_query::{FindAllProductsQuery, FindProductByIdQuery},
    },
    infrastructure::{
        http::State,
        interfaces::http::resources::{DataResponse, product_resource::ProductResource},
        persistence::sea_orm_product_repository::SeaOrmProductRepository,
    },
};

#[handler]
pub async fn create_product_handler(depot: &mut Depot, req: &mut Request, res: &mut Response)
/* -> AppResult<Json<DataResponse<ProductResource>>> */
{
    let state = depot.obtain::<Arc<State>>().unwrap();

    match req.parse_json::<ProductResource>().await {
        Ok(data) => {
            let repository = SeaOrmProductRepository::new(state.db.clone());

            match CreateProductUseCase::new(repository)
                .execute(data.name, data.price, data.sku, data.description)
                .await
            {
                Ok(data) => {
                    res.status_code(StatusCode::CREATED);
                    res.render(Json(DataResponse::success(ProductResource::from(&data))));
                }
                Err(err) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(DataResponse::error(err.to_string())));
                }
            }
        }
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(DataResponse::error(err.to_string())))
        }
    }
}

#[handler]
pub async fn index_product_handler(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<State>>().unwrap();
    let repository = SeaOrmProductRepository::new(state.db.clone());

    match FindAllProductsQuery::new(repository).execute().await {
        Ok(data) => res.render(Json(DataResponse::success(ProductResource::collection(
            data,
        )))),
        Err(err) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(DataResponse::error(err.to_string())));
        }
    }
}

#[handler]
pub async fn get_product_by_id_handler(
    depot: &mut Depot,
    req: &mut Request,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot.obtain::<Arc<State>>().unwrap();
    let repository = SeaOrmProductRepository::new(state.db.clone());

    let id = req.params().get("id").cloned().unwrap().parse::<i32>()?;

    match FindProductByIdQuery::new(repository).execute(id).await {
        Ok(data) => {
            res.status_code(StatusCode::OK);
            res.render(Json(DataResponse::success(ProductResource::from(&data))));
        }
        Err(err) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(DataResponse::error(err.to_string())));
        }
    };

    Ok(())
}