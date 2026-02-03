use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::plan_handler::create_plan_handler;

pub mod features;
pub fn router() -> Router {
    Router::with_path("plans")
        .post(create_plan_handler)
        .push(features::router())
}
