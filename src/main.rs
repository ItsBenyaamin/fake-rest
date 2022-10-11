use tokio::{net::TcpListener, io::AsyncReadExt};

mod server_config;
mod request;
mod response;



type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000").await?;
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = String::new();
        
        tokio::spawn(async move {
            let mut request_buf = String::new();
            socket.read_to_string(&mut request_buf).await.unwrap();

            let parsed_request = request::parse_request();
            let response = response::get_proper_response();
            
        });
    }

    Ok(())
}
