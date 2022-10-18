use std::path::PathBuf;

use serde::Deserialize;

use crate::server::request::Method;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub config: Config,
    pub data: Vec<ServerDataSchema>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: usize
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerDataSchema {
    pub method: Method,
    pub path: String,
    pub result_type: String,
    pub result: String,
    pub result_headers: Option<Vec<String>>,
    pub status_code: Option<usize>,
    pub headers: Option<Vec<String>>,
    pub queries: Option<Vec<String>>
}


pub async fn parse_config_file(path: PathBuf) -> tokio::io::Result<Server> {
    let content = tokio::fs::read_to_string(path).await?;
    let parsed_server = toml::from_str(&content)?;
    Ok(parsed_server)
}