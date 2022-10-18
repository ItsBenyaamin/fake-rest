use fake_rest::server_config::Server;
use tokio::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use clap::Parser;

mod error;
mod server;
mod fake_rest;

use error::FakeRestResult;
use crate::server::{
    response::Response, 
    connection::Connection, 
};
use crate::fake_rest::{
    server_config,
    print
};

const FAKE_REST: &str = r"
/$$$$$$$$       /$$                       /$$$$$$$                        /$$    
| $$_____/      | $$                      | $$__  $$                      | $$    
| $$    /$$$$$$ | $$   /$$  /$$$$$$       | $$  \ $$  /$$$$$$   /$$$$$$$ /$$$$$$  
| $$$$$|____  $$| $$  /$$/ /$$__  $$      | $$$$$$$/ /$$__  $$ /$$_____/|_  $$_/  
| $$__/ /$$$$$$$| $$$$$$/ | $$$$$$$$      | $$__  $$| $$$$$$$$|  $$$$$$   | $$    
| $$   /$$__  $$| $$_  $$ | $$_____/      | $$  \ $$| $$_____/ \____  $$  | $$ /$$
| $$  |  $$$$$$$| $$ \  $$|  $$$$$$$      | $$  | $$|  $$$$$$$ /$$$$$$$/  |  $$$$/
|__/   \_______/|__/  \__/ \_______/      |__/  |__/ \_______/|_______/    \___/  
                                                                                                                                                                                                                                               
";


#[derive(Parser)]
#[command(author, version, about, long_about = Some(
    "Fake Rest make development easier by serving a fake restApi server with given config file."
))]
pub struct FakeRestArgs {
    #[arg(short, long)]
    pub config: PathBuf
}


async fn handle(socket: TcpStream, server: &Server) -> FakeRestResult {
    let mut connection = Connection::new(socket).await?;
    let response = Response::new(&connection.request, &server).await?;
    connection.respond(response).await?;
    print::format_for_print(&connection.request);
    Ok(())
}


#[tokio::main]
async fn main() -> FakeRestResult {
    println!("{}", FAKE_REST);

    let args = FakeRestArgs::parse();
    let server = server_config::parse_config_file(args.config).await?;

    let host_and_port = format!("127.0.0.1:{}", server.config.port);
    let listener = TcpListener::bind(&host_and_port).await?;
    println!("Start the server at <http://{}>...", host_and_port);
    loop {
        let (socket, _) = listener.accept().await?;
        handle(socket, &server).await?;
    }
}
