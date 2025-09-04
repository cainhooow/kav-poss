mod routers;

use std::sync::Arc;

use crate::{infrastructure::database::estabilish_connection, routers::routers};
use salvo::{catcher::Catcher, prelude::*};
use sea_orm::DatabaseConnection;

mod application;
mod domain;
mod infrastructure;

#[derive(Default, Clone, Debug)]
pub struct State {
    db: Arc<DatabaseConnection>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let acceptor = TcpListener::new("0.0.0.0:5050").bind().await;
    _ = dotenvy::dotenv();

    let connection = estabilish_connection().await;
    let router = Router::with_path("api")
        .hoop(affix_state::inject(Arc::new(State {
            db: Arc::new(connection),
        })))
        .push(routers());

    let router_service = Service::new(router).catcher(Catcher::default());
    Server::new(acceptor).serve(router_service).await;
}
