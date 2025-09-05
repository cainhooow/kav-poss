use salvo::Router;

use crate::infrastructure::{
    http::middlewares::auth_middleware::AuthMiddleware,
    interfaces::http::handlers::product_handler::{
        create_product_handler, get_product_by_id_handler, index_product_handler,
    },
};

pub fn router() -> Router {
    Router::with_path("products")
        .hoop(AuthMiddleware)
        .get(index_product_handler)
        .post(create_product_handler)
        .push(Router::with_path("{id}").get(get_product_by_id_handler))
}
