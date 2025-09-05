use salvo::prelude::*;
pub mod product;
pub mod auth;

pub fn routers() -> Router {
    Router::with_path("v1")
        .hoop(Logger::new())
        .push(auth::router())
        .push(product::router())
}
