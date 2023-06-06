use serde::{Deserialize, Serialize};
use clap::Parser;
use salvo::{Listener, Router, Server};
use salvo::prelude::{handler, TcpListener};
use sp_web::config::log;

/// Rust简单web服务
#[derive(Parser, Debug, Deserialize, Serialize)]
struct Args {
    /// 端口号
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    bind: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let _guard = log::init_log();

    let app = Router::new().get(root);

    let addr = TcpListener::new(&args.bind).bind().await;
    tracing::info!("listening on {}", args.bind);
    Server::new(addr).serve(app).await;
}


#[handler]
async fn root() -> &'static str {
    "Hello, World!"
}
