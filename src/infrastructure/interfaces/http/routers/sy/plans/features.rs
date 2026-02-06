use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::plan_handler::get_plan_features_handler;

pub fn router() -> Router {
    Router::with_path("{plan_id}/features")
    .get(get_plan_features_handler)
}