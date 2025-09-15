use salvo::prelude::*;
pub mod auth;
pub mod product;

pub fn routers() -> Router {
    Router::with_path("v1")
        .hoop(Logger::new())
        .push(auth::router())
        .push(product::router())
}
