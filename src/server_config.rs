use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub config: Config,
    pub server: Vec<ServerDataSchema>
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: usize
}

#[derive(Debug, Deserialize)]
pub struct ServerDataSchema {
    pub method: String,
    pub path: String,
    pub result_type: String,
    pub result: String,
    pub status_code: usize,
    pub headers: Option<Vec<String>>,
    pub queries: Option<Vec<String>>
}


pub async fn parse_config_file(path: PathBuf) -> tokio::io::Result<Server> {
    let content = tokio::fs::read_to_string(path).await?;
    let parsed_server: Server = toml::from_str(&content).unwrap();
    Ok(parsed_server)
}