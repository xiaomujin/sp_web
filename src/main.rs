use clap::Parser;
use rusqlite::{params, Connection};
use salvo::logging::Logger;
use salvo::prelude::TcpListener;
use salvo::{Listener, Router, Server, Service};
use serde::{Deserialize, Serialize};
use sp_web::config::log;
use sp_web::constant::app;
use sp_web::controller;
use std::fmt::Debug;
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
    use app::GLOBAL_CONFIG;
    let port = args.port.unwrap_or(GLOBAL_CONFIG.server.port);
    let router = controller::init();
    let service = Service::new(router).hoop(Logger::new());
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
    use prost::Message;
    use sp_web::pb;
    use std::time::Instant;

    #[test]
    fn test_round_trip() {
        let mut role = pb::Role {
            item_list: Vec::new(),
        };
        let now1 = Instant::now();
        for i in 0..100000 {
            let original = pb::Item {
                id: i,
                num: 999,
                level: 99,
                star: 99,
            };
            role.item_list.push(original);
        }
        println!("{:?}", now1.elapsed());

        let now2 = Instant::now();
        let _vec1 = role.encode_to_vec();
        println!("{:?}", now2.elapsed());

        let now3 = Instant::now();
        let _role1 = pb::Role::decode(&_vec1[..]).unwrap();
        println!("{:?}", now3.elapsed());

        let now4 = Instant::now();
        let _json = serde_json::to_string(&role).unwrap();
        println!("{:?}", now4.elapsed());
    }
}
