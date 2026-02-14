use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::company_handler::create_company_role_handler;

pub fn router() -> Router {
    Router::with_path("roles").post(create_company_role_handler)
}
