use salvo::{
    http::{
        HeaderMap, HeaderValue,
        header::{CONTENT_TYPE, FORWARDED},
    },
    prelude::*,
};

pub struct AppMiddleware;

#[async_trait::async_trait]
impl Handler for AppMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(FORWARDED, HeaderValue::from_static("Kav-poss"));
        res.set_headers(headers);
        ctrl.call_next(req, depot, res).await;
    }
}
