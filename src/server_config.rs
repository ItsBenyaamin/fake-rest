use std::path::PathBuf;

use serde::{Serialize, Deserialize};


pub struct ServerSchema {
    pub endpoints: Vec<ServerDataSchema>
}

#[derive(Serialize, Deserialize)]
pub struct ServerDataSchema {
    pub path: String,
    pub result_type: String,
    pub headers: Vec<String>,
    pub result_path: Option<PathBuf>,
    pub result_string: Option<String>
}


pub async fn parse_config_file(path: PathBuf) -> tokio::io::Result<ServerSchema> {
    let content = tokio::fs::read_to_string(path).await?;
    let parsed_to_serde: Vec<ServerDataSchema> = serde_json::from_str(&content)?;
    Ok(
        ServerSchema { endpoints: parsed_to_serde }
    )
}