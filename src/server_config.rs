use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct ServerSchema {
    pub endpoints: Vec<ServerDataSchema>
}

#[derive(Deserialize)]
pub struct ServerDataSchema {
    pub method: String,
    pub path: String,
    pub result_type: String,
    pub result: String,
    pub status_code: usize,
    pub headers: Option<Vec<String>>,
    pub queries: Option<Vec<String>>
}


pub async fn parse_config_file(path: PathBuf) -> tokio::io::Result<ServerSchema> {
    let content = tokio::fs::read_to_string(path).await?;
    let parsed_to_serde: Vec<ServerDataSchema> = toml::from_str(&content).unwrap();
    Ok(
        ServerSchema { endpoints: parsed_to_serde }
    )
}