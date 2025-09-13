use crate::infrastructure::{http::State, interfaces::http::resources::DataResponse};
use salvo::prelude::*;
use std::env;
use std::sync::Arc;
pub struct AuthMiddleware;

#[async_trait::async_trait]
impl Handler for AuthMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        if let Some(auth_header) = req.headers().get("Authorization") {
            let state = depot.obtain::<Arc<State>>().unwrap();
            let auth_header = auth_header.to_str().unwrap_or("");

            match state.auth_service.validate_from_authorization(auth_header) {
                Ok(auth) => match state.auth_service.validate_token(auth.token) {
                    Ok(claims) => {
                        depot.insert("user_id", claims.sub);
                        ctrl.call_next(req, depot, res).await;
                    }
                    Err(_) => {
                        res.render(DataResponse::error("Invalid access token"));
                        res.status_code(StatusCode::UNAUTHORIZED);
                    }
                },
                Err(err) => {
                    res.render(DataResponse::error("Invalid access token"));
                    res.status_code(StatusCode::UNAUTHORIZED);
                }
            }
        } else if let Some(auth_cookie) = req.cookie("kvsession") {
            let token = auth_cookie.value();
            let state = depot.obtain::<Arc<State>>().unwrap();

            match state.auth_service.validate_token(token) {
                Ok(claims) => {
                    depot.insert("user_id", claims.sub);
                    ctrl.call_next(req, depot, res).await;
                }
                Err(_) => {
                    res.render(DataResponse::error("Invalid access token cookie"));
                    res.status_code(StatusCode::UNAUTHORIZED);
                }
            }
        } else {
            res.render(DataResponse::error("Access token absent"));
            res.status_code(StatusCode::UNAUTHORIZED);
        }
    }
}
