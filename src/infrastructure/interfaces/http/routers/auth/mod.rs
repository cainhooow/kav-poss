use salvo::Router;

use crate::infrastructure::{
    http::middlewares::auth_middleware::AuthMiddleware,
    interfaces::http::handlers::auth_handler::{
        auth_local_login, auth_local_refresh, auth_local_register, auth_user,
    },
};

pub fn router() -> Router {
    Router::with_path("auth")
        .push(
            Router::with_path("user")
                .hoop(AuthMiddleware)
                .get(auth_user),
        )
        .push(
            Router::with_path("local")
                .post(auth_local_register)
                .push(Router::with_path("login").post(auth_local_login))
                .push(Router::with_path("refresh").post(auth_local_refresh)),
        )
}
