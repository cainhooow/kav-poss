use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::auth_handler::{
    auth_local, auth_local_register,
};

pub fn router() -> Router {
    Router::with_path("auth").push(
        Router::with_path("local")
            .get(auth_local)
            .post(auth_local_register),
    )
}
