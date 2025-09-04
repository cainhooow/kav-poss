use salvo::{http::StatusCode, writing::Json, Depot, Request, Response, Writer};

use crate::{application::exceptions::AppError, infrastructure::interfaces::http::resources::DataResponse};

#[async_trait::async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        let (status, message) = match &self {
            AppError::Domain(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Repository(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unexpected(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        res.status_code(status);
        res.render(Json(DataResponse::error(message)));
    }
}
