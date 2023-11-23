use anyhow::anyhow;
use reqwest::header::HeaderValue;
use salvo::{handler, Request, Response, Router};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tracing::instrument;

#[instrument]
pub fn init() -> Router {
    Router::with_path("file").get(get_file)
}


#[handler]
async fn get_file(req: &mut Request, res: &mut Response) {
    let url = req.query::<String>("url").unwrap_or_default();
    let response = reqwest::get(url).await.unwrap();
    // save("1.png", &mut response).await.unwrap();
    let header_value = HeaderValue::from_str("text/plain").unwrap();
    let content_type = response.headers().get("content-type").unwrap_or(&header_value);
    res.add_header("content-type", content_type, true).unwrap();
    // res.write_body(response.bytes().await.unwrap()).unwrap();
    let stream = response.bytes_stream();
    res.stream(stream);
}

async fn save(filename: &str, response: &mut reqwest::Response) -> Result<(), Box<dyn std::error::Error>> {
    let mut options = OpenOptions::new();
    let mut file = options
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)
        .await.unwrap();

    while let Some(chunk) = &response.chunk().await.expect("Failed") {
        match file.write_all(&chunk).await {
            Ok(_) => {}
            Err(e) => return Err(Box::try_from(anyhow!("File {} save error: {}", filename, e)).unwrap())
        }
    }
    Ok(())
}