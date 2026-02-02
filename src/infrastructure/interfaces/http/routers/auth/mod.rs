use salvo::Router;

use crate::infrastructure::{
    http::middlewares::auth_middleware::AuthMiddleware,
    interfaces::http::handlers::auth_handler::{
        auth_local_login_handler, auth_local_refresh_handler, auth_local_register_handler,
        auth_user_handler,
    },
};

pub fn router() -> Router {
    Router::with_path("auth")
        .push(
            Router::with_path("user")
                .hoop(AuthMiddleware)
                .get(auth_user_handler),
        )
        .push(
            Router::with_path("local")
                .post(auth_local_register_handler)
                .push(Router::with_path("login").post(auth_local_login_handler))
                .push(Router::with_path("refresh").post(auth_local_refresh_handler)),
        )
}
