use salvo::Router;

use crate::infrastructure::{http::middlewares::auth_middleware::AuthMiddleware, interfaces::http::handlers::auth_handler::auth_user};

pub mod company;

pub fn router() -> Router {
    Router::with_path("me")
        .hoop(AuthMiddleware)
        .get(auth_user)
        .push(company::router())
}
