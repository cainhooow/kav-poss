use salvo::{handler, Depot, FlowCtrl, Request, Response};


#[handler]
pub async fn global_catcher(_req: &mut Request, _res: &mut Response, _ctrl: &mut FlowCtrl, _depot: &mut Depot) {
    
}