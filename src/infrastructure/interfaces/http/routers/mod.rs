pub use salvo::prelude::*;

pub mod auth;
pub mod company;
pub mod me;
pub mod product;
pub mod sy;

pub fn routers() -> Router {
    Router::with_path("v1")
        .push(auth::router())
        .push(me::router())
        .push(product::router())
        .push(sy::router())
        .push(company::router())
}
