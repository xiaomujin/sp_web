use anyhow::anyhow;
use reqwest::header::HeaderValue;
use salvo::{Request, Response, Router, handler};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tracing::instrument;

#[instrument]
pub fn init() -> Router {
    Router::with_path("file/{**url}")
        .head(head_file)
        .get(get_file)
}

#[handler]
async fn head_file(req: &mut Request, res: &mut Response) {
    let url = "https://".to_owned()
        + req
            .uri()
            .path_and_query()
            .unwrap()
            .to_string()
            .strip_prefix("/file/")
            .unwrap();
    let header_value_str = &get_content_type(url);
    let header_value = HeaderValue::from_str(header_value_str).unwrap();
    res.add_header("content-type", header_value, true).unwrap();
}

fn get_content_type(url: String) -> String {
    let mut header_value_str = "text/plain";
    if url.contains("png") {
        header_value_str = "image/png";
    } else if url.contains("jpg") {
        header_value_str = "image/jpeg";
    } else if url.contains("gif") {
        header_value_str = "image/gif";
    }
    header_value_str.to_string()
}

#[handler]
async fn get_file(req: &mut Request, res: &mut Response) {
    let url = req.param::<String>("url").unwrap_or_default();
    tracing::info!("get_file url: {}", url);
    let url = "https://".to_owned()
        + req
            .uri()
            .path_and_query()
            .unwrap()
            .to_string()
            .strip_prefix("/file/")
            .unwrap();
    let header_value_str = &get_content_type(url.to_owned());
    let response = reqwest::get(url).await.unwrap();
    // // save("1.png", &mut response).await.unwrap();
    let header_value = HeaderValue::from_str(header_value_str).unwrap();
    // let content_type = response.headers().get("content-type").unwrap_or(&header_value);
    res.add_header("content-type", header_value, true).unwrap();
    res.write_body(response.bytes().await.unwrap()).unwrap();
    // let stream = response.bytes_stream();
    // res.stream(stream);
}

#[allow(dead_code)]
async fn save(
    filename: &str,
    response: &mut reqwest::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut options = OpenOptions::new();
    let mut file = options
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)
        .await
        .unwrap();

    while let Some(chunk) = &response.chunk().await.expect("Failed") {
        match file.write_all(&chunk).await {
            Ok(_) => {}
            Err(e) => {
                return Err(Box::try_from(anyhow!("File {} save error: {}", filename, e)).unwrap());
            }
        }
    }
    Ok(())
}
