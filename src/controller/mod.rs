mod file_controller;
mod root_controller;
mod bat_controller;
mod bin_controller;

use salvo::Router;
use tracing::instrument;

#[instrument]
pub fn init() -> Router {
    tracing::info!("注册路由");
    let router = Router::new()
        .push(root_controller::init())
        .push(bat_controller::init())
        .push(bin_controller::init())
        .push(file_controller::init());
    tracing::info!(router=?router);
    router
}
