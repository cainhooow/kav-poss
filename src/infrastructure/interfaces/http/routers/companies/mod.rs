use salvo::Router;

pub mod colaborators;
pub mod products;
pub mod roles;

pub fn router() -> Router {
    Router::with_path("companies/{company_id}")
        .push(colaborators::router())
        .push(roles::router())
        .push(products::router())
}
