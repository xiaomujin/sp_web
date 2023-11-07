use lazy_static::lazy_static;
use crate::config::models::GlobalConfig;
use crate::config::init_load_config;


lazy_static! {
    pub static ref GLOBAL_CONFIG:GlobalConfig=init_load_config::load_global_config().unwrap();
}