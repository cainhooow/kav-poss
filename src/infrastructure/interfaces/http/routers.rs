use salvo::prelude::*;

use crate::infrastructure::interfaces::http::handlers::product_handler::product_router;

pub fn routers() -> Router {
    Router::with_path("v1")
        .hoop(Logger::new())
        .push(product_router())
}
