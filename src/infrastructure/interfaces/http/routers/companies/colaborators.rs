use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::company_handler::create_company_colaborator_handler;

pub fn router() -> Router {
    Router::with_path("colaborators").post(create_company_colaborator_handler)
}
