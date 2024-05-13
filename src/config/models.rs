use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub mysql: Mysql,
    pub server: Server,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mysql {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db_name: String,
}
