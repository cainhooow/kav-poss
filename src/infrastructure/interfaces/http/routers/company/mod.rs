use salvo::Router;

pub mod colaborators;
pub mod products;

pub fn router() -> Router {
    Router::with_path("companies")
        .push(colaborators::router())
        .push(products::router())
}
