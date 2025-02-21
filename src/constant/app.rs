use crate::config::global_config;
use crate::config::models::GlobalConfig;
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

pub static GLOBAL_CONFIG: LazyLock<GlobalConfig> = LazyLock::new(global_config::load_global_config);
pub static GLOBAL_SET: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));
