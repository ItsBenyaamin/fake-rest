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
    match Response::new(&connection.request, &server).await {
        Ok(response) => {
            connection.respond(response).await?;
            print::format_for_print(&connection.request);
        },
        Err(e) => {
            println!("Error on handling the request: {}", e);
        },
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    println!("{}", FAKE_REST);

    let args = FakeRestArgs::parse();
    let server = match server_config::parse_config_file(args.config).await {
        Ok(s) => s,
        Err(e) => panic!("{}", e.to_string())
    };

    let host_and_port = format!("{}:{}", server.config.host, server.config.port);
    let listener = match TcpListener::bind(&host_and_port).await {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };

    println!("Start the server at <http://{}>...", host_and_port);
    loop {
        let con = listener.accept().await;
        if let Ok(connection) = con {
            match handle(connection.0, &server).await {
                Ok(_) => {},
                Err(e) => println!("{}", e.to_string())
            };
        }else {
            println!("{}", con.err().unwrap().to_string())
        }
    }
}
