use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::company_handler::{create_company_colaborator_handler, get_company_colaborators_handler};

pub fn router() -> Router {
    Router::with_path("colaborators")
    .get(get_company_colaborators_handler)
    .post(create_company_colaborator_handler)
}
