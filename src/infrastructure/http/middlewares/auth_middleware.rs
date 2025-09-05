use std::sync::Arc;

use salvo::prelude::*;

use crate::infrastructure::{http::State, interfaces::http::resources::DataResponse};
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
            let token = auth_header.to_str().unwrap_or("").replace("Bearer", "");
            let state = depot.obtain::<Arc<State>>().unwrap();

            match state.auth_service.validate_token(&token) {
                Ok(claims) => {
                    depot.insert("user_id", claims.sub);
                    ctrl.call_next(req, depot, res).await;
                }
                Err(_) => {
                    res.render(Json(DataResponse::error("Invalid access token")));
                    res.status_code(StatusCode::UNAUTHORIZED);
                }
            }
        } else {
            res.render(Json(DataResponse::error("Token absent")));
            res.status_code(StatusCode::UNAUTHORIZED);
        }
    }
}
