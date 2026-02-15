use salvo::Router;

use crate::infrastructure::http::middlewares::auth_middleware::AuthMiddleware;

pub mod colaborators;
pub mod products;
pub mod roles;

pub fn router() -> Router {
    Router::with_path("companies/{company_id}")
        .hoop(AuthMiddleware)
        .push(colaborators::router())
        .push(roles::router())
        .push(products::router())
}
