use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub sqlite: Sqlite,
    pub server: Server,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sqlite {
    pub db_name: String,
}
