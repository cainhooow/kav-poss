use salvo::prelude::*;
pub mod product_router;
use self::product_router::product_router;

pub fn routers() -> Router {
    Router::with_path("v1")
        .hoop(Logger::new())
        .push(product_router())
}
