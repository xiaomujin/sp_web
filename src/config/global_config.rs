use crate::config::models::GlobalConfig;
use rust_embed::Embed;
use schemars::schema::RootSchema;
use serde::de::DeserializeOwned;
use std::io::Write;
use std::path::Path;
use std::{fs, process};

#[derive(Embed)]
#[folder = "resource"]
struct Resource;

fn get_resource(path: &str) -> Option<rust_embed::EmbeddedFile> {
    Resource::get(path)
}

pub fn load_global_config() -> GlobalConfig {
    load_config::<GlobalConfig>("application.yml").unwrap()
}

fn load_config<T>(path: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    if !Path::new(path).exists() {
        let in_file = get_resource(path).unwrap();
        let mut file = fs::File::create(path).expect("create failed");
        file.write_all(in_file.data.as_ref()).expect("write failed");
        tracing::info!("{path} 文件已创建，修改后重新启动");
        process::exit(0x0000);
    }
    let cfg_str = fs::read_to_string(path).unwrap_or_else(|e| panic!("{path} {e}"));
    match serde_yaml::from_str::<RootSchema>(&cfg_str) {
        Ok(root_schema) => {
            let data = serde_json::to_string_pretty(&root_schema).expect("");
            let config = serde_json::from_str::<T>(&data).expect("");
            Some(config)
        }
        Err(err) => {
            tracing::error!("{}", err);
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::global_config::load_global_config;

    #[test]
    pub fn load_config_test() {
        load_global_config();
    }
}
