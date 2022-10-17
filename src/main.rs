use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use clap::Parser;

mod error;
mod server_config;
mod server;

use error::FakeRestResult;
use crate::server::{response::{Status, Response}, connection::Connection};
use server::request::Request;

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


async fn handle(socket: TcpStream) -> FakeRestResult {
    let mut connection = Connection::new(socket).await?;
    format_for_print(&connection.request);
    
    let body = String::from("Hello, World.");
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("Content-Length".to_string(), body.len().to_string());
    if let Some(host) = connection.request.headers.get("Host") {
        headers.insert("Host".to_string(), host.to_string());
    }

    let response = Response {
        status: Status::OK(),
        headers,
        body
    };
    connection.respond(response).await?;
    
    Ok(())
}


#[tokio::main]
async fn main() -> FakeRestResult {
    println!("{}", FAKE_REST);

    let args = FakeRestArgs::parse();
    let server_config = server_config::parse_config_file(args.config).await?;

    let host_and_port = format!("127.0.0.1:{}", server_config.config.port);
    let listener = TcpListener::bind(&host_and_port).await?;
    println!("Start the server at <http://{}>...", host_and_port);
    loop {
        let (socket, _) = listener.accept().await?;
        handle(socket).await?;
    }
}

fn format_for_print(request: &Request) {
    let mut query_strings = String::new();
    if request.query_strings.len() == 0 {
        query_strings.push_str("\n-      -- Empty --");
    }else {
        for query in request.query_strings.iter() {
            let query_style = format!("\n-       -{} = {}", query.0, query.1);
            query_strings.push_str(&query_style);
        }
    }
    
    let mut headers = String::new();
    for header in request.headers.iter(){
        let header_style = format!("\n-      -{} : {}", header.0, header.1);
        headers.push_str(&header_style);
    }
    println!("");
    println!("------------------------ Start Request-------------------------");
    let printable = format!("-- Version: {}\n-- Type: {}\n-- Path: {}\n-- Query Strings:{}\n-- Headers:{}", request.version, request.method, request.uri, query_strings, headers);
    println!("{}", printable);
    println!("------------------------ End  Request-------------------------");
}
