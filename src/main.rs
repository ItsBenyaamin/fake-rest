use std::path::PathBuf;
use clap::Parser;

mod server_config;
mod request;
mod response;

#[derive(Parser)]
#[command(author, version, about, long_about = Some(
    "Fake Rest make development easier by serving a fake restApi server with given config file."
))]
pub struct FakeRestArgs {
    #[arg(short, long)]
    pub config: PathBuf
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[tokio::main]
async fn main() -> Result<()> {
    let args = FakeRestArgs::parse();
    let _server_config = server_config::parse_config_file(args.config).await?;

    Ok(())
}
