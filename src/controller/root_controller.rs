use salvo::http::{header, HeaderValue};
use salvo::prelude::Text;
use salvo::{handler, Response, Router};
use tracing::instrument;

#[instrument]
pub fn init() -> Router {
    Router::with_hoop(add_header).goal(root)
}

#[handler]
async fn add_header(res: &mut Response) {
    res.headers_mut()
        .insert(header::SERVER, HeaderValue::from_static("Salvo"));
}

#[handler]
async fn root(res: &mut Response) {
    res.render(Text::Html(INDEX_HTML));
}

static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>朱哥nb</title>
    </head>
    <body style="background-color: green;">
        <h1 style="text-align: center;">朱哥nb!</h1>
    </body>
</html>
"#;
