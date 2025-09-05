use std::{env, sync::Arc};

use salvo::prelude::*;
use sea_orm::DatabaseConnection;

#[derive(Default, Clone, Debug)]
pub struct State {
    pub db: Arc<DatabaseConnection>,
    pub auth_service: Arc<JwtAuthService>
}

use crate::infrastructure::{database::estabilish_connection, interfaces::http::routers::routers, services::jwt_auth_service::JwtAuthService};

async fn create_state() -> Arc<State> {
    let connection = estabilish_connection().await;
    let jwt_secret = env::var("JWT_AUTH_SECRET").expect("JWT_AUTH_SECRET NOT PROVIDED");
    
    Arc::new(State {
        db: Arc::new(connection),
        auth_service: Arc::new(JwtAuthService::new(jwt_secret))
    })
}

fn create_router(state: Arc<State>) -> Router {
    Router::with_path("api")
        .hoop(affix_state::inject(state))
        .push(routers())
}

pub async fn http_server_init() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("0.0.0.0:5050").bind().await;
    let state = create_state().await;
    let router = create_router(state);

    Server::new(acceptor).serve(router).await;
}