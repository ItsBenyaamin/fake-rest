use tokio::{net::TcpStream, io::AsyncWriteExt};
use crate::error;
use super::{request::Request, response::Response};


pub struct Connection {
    pub request: Request,
    pub socket: TcpStream
}

impl Connection {
    
    pub async fn new(mut socket: TcpStream) -> Result<Connection, error::Error> {
        let request = Request::new(&mut socket).await?;

        Ok(Connection {
            request,
            socket
        })
    }

    pub async fn respond(&mut self, response: Response) -> Result<(), error::Error> {
        self.socket.write_all(format!("HTTP/1.1 {} {}\r\n", response.status.code, response.status.message).as_bytes()).await?;
        for (k, v) in response.headers.iter() {
            self.socket.write_all(format!("{} : {}\r\n", k, v).as_bytes()).await?;
        }
        self.socket.write_all(b"\r\n").await?;
        if response.body.len() != 0 {
            self.socket.write_all(format!("{}\r\n", response.body).as_bytes()).await?;
        }

        Ok(())
    }

}