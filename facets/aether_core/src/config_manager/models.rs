use serde::{Deserialize, Serialize};
use std::fs;
use toml::de::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub pool_size: Option<u32>,
    pub ssl_mode: Option<String>,
    pub db_filter: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AetherConfig {
    pub database: Option<DatabaseConfig>,
    pub server: ServerConfig,
}

impl Default for AetherConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {},
            server: ServerConfig {
                host: "localhost".to_string(),
                port: 7890,
            },
        }
    }
}

impl AetherConfig {
    pub fn gen_config_from_file(file_path: &str) -> Result<Error, Self> {
        let data = fs::read_to_string(file_path);
    }
}

pub fn gen_config_file(file_path: &str) -> Result<Error, String> {
    Ok("")
}
