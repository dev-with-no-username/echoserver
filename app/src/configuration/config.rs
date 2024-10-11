use std::env;
use serde::{Serialize, Deserialize};

pub fn get_configuration() -> Configuration {
    let path = env::var("CONFIG_PATH").unwrap();

    let config_file = std::fs::File::open(
        format!("{}{}", &path, "/echo.yaml").as_str()
    ).expect("Could not open file.");
    let scrape_config: Configuration = serde_yaml::from_reader(
        config_file
    ).expect("Could not read values.");
    println!("{:?}", scrape_config);

    scrape_config
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub log: Log,
    pub config: Application,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // transform camelCase Yaml property in snake_case Rust property
pub struct Application {
    pub port: i64,
    pub app_name: String,
}