use crate::constant::app::GLOBAL_CONFIG;
use crate::constant::app::GLOBAL_SET;
use salvo::{Request, Response, Router, handler};
use tokio::time::sleep;
use tracing::instrument;

#[instrument]
pub fn init() -> Router {
    Router::with_path("bin")
        .get(count_all)
        .push(Router::with_path("{**path}").get(count_file))
}

#[handler]
async fn count_file(req: &mut Request, res: &mut Response) {
    let path = req.uri().path();
    tracing::info!("url: {}", path);
    let url = GLOBAL_CONFIG.proxy.host.clone() + path;
    let mut bytes = Default::default();
    match reqwest::get(&url).await {
        Ok(res) => {
            bytes = res.bytes().await.unwrap();
        }
        _ => {
            tracing::warn!("超时 {}", &url);
        }
    };
    sleep(std::time::Duration::from_millis(GLOBAL_CONFIG.proxy.time)).await;
    let mut guard = GLOBAL_SET.lock().unwrap();
    guard.insert(path.to_string());
    res.write_body(bytes).unwrap();
}
#[handler]
async fn count_all(res: &mut Response) {
    let guard = GLOBAL_SET.lock().unwrap();
    let vec = serde_json::to_vec(&guard.clone()).unwrap();
    res.write_body(vec).unwrap();
}
