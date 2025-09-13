use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::auth_handler::{
    auth_local_login, auth_local_register,
};

pub fn router() -> Router {
    Router::with_path("auth").push(
        Router::with_path("local")
            .post(auth_local_register)
            .push(Router::with_path("/login").post(auth_local_login)),
    )
}
