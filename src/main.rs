use clap::Parser;
use salvo::logging::Logger;
use salvo::prelude::TcpListener;
use salvo::{Listener, Router, Server, Service};
use serde::{Deserialize, Serialize};
use sp_web::config::log;
use sp_web::constant::app;
use sp_web::controller;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

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
    let router = controller::init();
    let service = Service::new(router).hoop(Logger::new());
    // let server = run_server(port, router);
    let server = start_server_with_port(port, service);
    tracing::info!("listening on port: {}", port);
    server.await;
}

#[allow(dead_code)]
#[cfg(windows)]
async fn run_server(port: u16, router: Router) {
    let socket_v4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let socket_v6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);
    let addr = TcpListener::new(socket_v4)
        .join(TcpListener::new(socket_v6))
        .bind()
        .await;
    Server::new(addr).serve(router).await;
}

#[allow(dead_code)]
#[cfg(not(windows))]
async fn run_server(port: u16, router: Router) {
    let socket_v6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);
    let addr = TcpListener::new(socket_v6).bind().await;
    Server::new(addr).serve(router).await;
}

async fn start_server_with_port(port: u16, service: Service) {
    // tracing::info!("try on port: {}", port);
    let socket_v4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let socket_v6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);
    match TcpListener::new(socket_v4)
        .join(TcpListener::new(socket_v6))
        .try_bind()
        .await
    {
        Ok(addr) => {
            Server::new(addr).serve(service).await;
        }
        Err(_) => match TcpListener::new(socket_v6).try_bind().await {
            Ok(addr) => {
                Server::new(addr).serve(service).await;
            }
            Err(_) => {
                let addr = TcpListener::new(socket_v4).bind().await;
                Server::new(addr).serve(service).await;
            }
        },
    };
}
