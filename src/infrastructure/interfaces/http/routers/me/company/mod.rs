use salvo::Router;

use crate::infrastructure::interfaces::http::handlers::company_handler::create_company_handler;

pub mod colaborators;
pub mod products;

pub fn router() -> Router {
    Router::with_path("company")
        .post(create_company_handler)
        .push(colaborators::router())
        .push(products::router())
}
