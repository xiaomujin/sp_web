use schemars::schema::RootSchema;
use serde::de::DeserializeOwned;
use crate::config::models::{EnvConfig, GlobalConfig};

fn load_env_config() -> Option<EnvConfig> {
    load_config::<EnvConfig>("application.yml")
}

fn load_global_config_from_env(active: String) -> Option<GlobalConfig> {
    let path = format!("application-{active}.yml");
    load_config::<GlobalConfig>(&path)
}

pub fn load_global_config() -> Option<GlobalConfig> {
    if let Some(env_config) = load_env_config() {
        return load_global_config_from_env(env_config.profiles.active);
    }
    None
}

fn load_config<T>(path: &str) -> Option<T> where T: DeserializeOwned {
    let cfg_str = std::fs::read_to_string(path).expect(&format!("failure read file {path}"));
    match serde_yaml::from_str::<RootSchema>(&cfg_str) {
        Ok(root_schema) => {
            let data = serde_json::to_string_pretty(&root_schema).expect("");
            let config = serde_json::from_str::<T>(&data).expect("");
            Some(config)
        }
        Err(err) => {
            tracing::error!("{}",err);
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::global_config::load_global_config;

    #[test]
    pub fn load_config_test() {
        match load_global_config() {
            None => {
                println!("None");
            }
            Some(config) => {
                println!("{:#?}", config);
            }
        }
    }
}