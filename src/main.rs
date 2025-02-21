use clap::Parser;
use rusqlite::{params, Connection};
use salvo::prelude::TcpListener;
use salvo::{Listener, Router, Server, Service};
use serde::{Deserialize, Serialize};
use sp_web::config::log;
use sp_web::constant::app;
use sp_web::controller;
use std::fmt::Debug;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
use salvo::logging::Logger;

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
    use app::GLOBAL_CONFIG;
    let port = args.port.unwrap_or(GLOBAL_CONFIG.server.port);
    let router = controller::init();
    // let service = Service::new(router).hoop(Logger::new());
    let service = Service::new(router);
    // let server = run_server(port, router);
    let server = start_server_with_port(port, service);
    tracing::info!("listening on port: {}", port);
    do_query(GLOBAL_CONFIG.sqlite.db_name.clone());
    server.await;
}

fn do_query(db_name: String) {
    let connection = Connection::open(db_name).unwrap();
    connection
        .execute(
            "CREATE TABLE if not exists person (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER
         )",
            params![],
        )
        .unwrap();
    // connection
    //     .execute(
    //         "INSERT INTO person (name, age) VALUES (?1, ?2)",
    //         params!["Alice", 32],
    //     )
    //     .unwrap();
    let mut stmt = connection
        .prepare("SELECT id, name, age FROM person WHERE age > ?")
        .unwrap();
    let rows = stmt
        .query_map(params![20], |row| {
            Ok((
                row.get::<_, i64>(0).unwrap(),
                row.get::<_, String>(1).unwrap(),
                row.get::<_, i64>(2).unwrap(),
            ))
        })
        .unwrap();

    for row in rows {
        let (id, name, age) = row.unwrap();
        tracing::info!("id: {}, name: {}, age: {}", id, name, age);
    }
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

#[cfg(test)]
mod tests {
    use tokio::task::JoinSet;

    #[tokio::test]
    async fn test_round_trip() {
        let max_concurrent_requests = 2000; // 设置最大并发请求数量
        let mut join_set = JoinSet::new();

        for i in 0..20000 {
            if join_set.len() >= max_concurrent_requests {
                // 等待至少一个任务完成
                while join_set.join_next().await.is_some() {}
            }

            join_set.spawn(async move {
                match reqwest::get("http://10.0.0.65:8080/game/Test").await {
                    Ok(response) => {
                        println!("{}-{}", i, response.text().await.unwrap())
                    }
                    Err(e) => {
                        println!("{}-{}", i, e.to_string())
                    }
                }
            });
        }

        // 等待所有任务完成
        while join_set.join_next().await.is_some() {}
    }
}
