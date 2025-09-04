use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::product_handler::{
    create_product_handler, get_product_by_id_handler, index_product_handler,
};

pub fn router() -> Router {
    Router::with_path("products")
        .get(index_product_handler)
        .post(create_product_handler)
        .push(Router::with_path("{id}").get(get_product_by_id_handler))
}
