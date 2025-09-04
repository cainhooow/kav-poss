use salvo::prelude::*;

use crate::infrastructure::interfaces::controllers::product_api::product_router;

pub fn routers() -> Router {
    Router::with_path("v1")
        .hoop(Logger::new())
        .push(product_router())
}
