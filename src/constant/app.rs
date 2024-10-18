use crate::config::global_config;
use crate::config::models::GlobalConfig;
use std::sync::LazyLock;

pub static GLOBAL_CONFIG: LazyLock<GlobalConfig> = LazyLock::new(global_config::load_global_config);
