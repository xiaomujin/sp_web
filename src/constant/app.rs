use crate::config::global_config;
use crate::config::models::GlobalConfig;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_CONFIG: GlobalConfig = global_config::load_global_config().unwrap();
}
