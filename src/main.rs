use tokio::net::TcpListener;

mod error;
mod server_config;
mod request;
mod response;


#[tokio::main]
async fn main() -> error::FakeRestResult {
    let listener = TcpListener::bind("127.0.0.1:7000").await?;
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = String::new();
        
        tokio::spawn(async move {
            let request = request::parse_request(&socket)?;
        });
    }

}
