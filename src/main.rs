use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
use serde::{Deserialize, Serialize};
use clap::Parser;
use salvo::{Listener, Router, Server};
use salvo::prelude::{handler, TcpListener};
use sp_web::config::{log};
use sp_web::constant::app;

/// Rust简单web服务
#[derive(Parser, Debug, Deserialize, Serialize)]
struct Args {
    /// 绑定地址
    #[arg(short, long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let _guard = log::init_log();
    let config = &app::GLOBAL_CONFIG;
    let port = args.port.unwrap_or(config.server.port);

    let app = Router::new().get(root);
    let socket_v4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let socket_v6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);
    let addr = TcpListener::new(socket_v4).join(TcpListener::new(socket_v6)).bind().await;
    let server = Server::new(addr).serve(app);
    tracing::info!("listening on port: {}", port);
    server.await;
}


#[handler]
async fn root() -> &'static str {
    "朱哥nb!"
}
