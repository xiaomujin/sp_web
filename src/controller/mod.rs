mod root_controller;

use salvo::Router;
use tracing::instrument;

#[instrument]
pub fn init() -> Router {
    tracing::info!("注册路由");
    let router = Router::new().push(root_controller::init());
    tracing::info!(router=?router);
    router
}