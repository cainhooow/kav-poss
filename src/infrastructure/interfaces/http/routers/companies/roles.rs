use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::company_handler::{create_company_role_handler, get_company_roles_handler};

pub fn router() -> Router {
    Router::with_path("roles")
    .get(get_company_roles_handler)
    .post(create_company_role_handler)
}
