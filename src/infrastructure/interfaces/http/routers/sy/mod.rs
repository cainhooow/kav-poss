use salvo::Router;

pub mod plans;

pub fn router() -> Router {
    Router::with_path("sy").push(plans::router())
}
