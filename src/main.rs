use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
use serde::{Deserialize, Serialize};
use clap::Parser;
use salvo::{Listener, Server};
use salvo::prelude::TcpListener;
use sp_web::config::{log};
use sp_web::constant::app;
use sp_web::controller;

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
    tracing::info!("try on port: {}", port);
    let socket_v4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    // let socket_v6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);
    let addr = TcpListener::new(socket_v4).bind().await;
    let router = controller::init();
    let server = Server::new(addr).serve(router);
    tracing::info!("listening on port: {}", port);
    server.await;
}