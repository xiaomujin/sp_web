use salvo::http::{header, HeaderValue};
use salvo::{handler, Response, Router};
use tracing::instrument;

#[instrument]
pub fn init() -> Router {
    Router::with_hoop(add_header).get(hello_world)
}

#[handler]
async fn add_header(res: &mut Response) {
    res.headers_mut()
        .insert(header::SERVER, HeaderValue::from_static("Salvo"));
}

#[handler]
async fn hello_world() -> &'static str {
    "朱哥nb!"
}
