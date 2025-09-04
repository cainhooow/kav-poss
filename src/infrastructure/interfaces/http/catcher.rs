use salvo::{handler, http::StatusCode, Depot, FlowCtrl, Request, Response};


#[handler]
async fn global_catcher(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl, depot: &mut Depot) {
    
}