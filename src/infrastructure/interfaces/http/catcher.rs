use salvo::{handler, http::StatusCode, Depot, FlowCtrl, Request, Response};


#[handler]
pub async fn global_catcher(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) {
    
}