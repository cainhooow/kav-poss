use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::product_handler::create_product_handler;

pub fn router() -> Router {
    Router::with_path("products")
    .post(create_product_handler)
}
