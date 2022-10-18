use std::{collections::HashMap, fmt::Display};
use serde::Deserialize;
use tokio::{net::TcpStream, io::AsyncReadExt};

use crate::error::Error;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    OPTION,
    DELETE
}

impl From<String> for Method {
    fn from(s: String) -> Self {
        return match s.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "OPTION" => Method::OPTION,
            "DELETE" => Method::DELETE,
            _=>  Method::GET
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::PATCH => write!(f, "PATCH"),
            Method::OPTION => write!(f, "OPTION"),
            Method::DELETE => write!(f, "DELETE"),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub query_strings: HashMap<String, String>,
}

impl Request {

    pub async fn new(reader: &mut TcpStream) -> crate::error::RequestParseResult {
        let mut request_info = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut buff: Vec<u8> = vec![];

        loop {
            let byte = reader.read_u8().await?;
            buff.push(byte);
            if byte as char == '\n' {
                if request_info.is_empty() {
                    request_info = String::from_utf8(buff[..buff.len() - 2].to_vec())?;
                    buff.clear();
                }else {
                    if buff.len() == 2 && buff[0] as char == '\r' {
                        break;
                    }

                    let header_line = String::from_utf8(buff[..buff.len() - 2].to_vec())?;
                    buff.clear();
                    let mut iter = header_line.split(':');
                    let key = match iter.next() {
                        Some(key) => key,
                        None => return Err(Error::ParsingError)
                    };

                    let value = match iter.next() {
                        Some(value) => value.trim(),
                        None => return Err(Error::ParsingError)
                    };

                    headers.insert(key.to_string(), value.to_string());
                }
            }
        }

        let mut request_info_iter = request_info.split(' ');
        let method: Method = request_info_iter.next().unwrap_or("").to_string().into();
        let uri = request_info_iter.next().unwrap_or("").to_string();
        let version = request_info_iter.next().unwrap_or("").to_string();
        
        let mut uri_iter = uri.split('?');
        let uri = match uri_iter.next() {
            Some(uri) => uri.to_string(),
            None => return Err(Error::ParsingError),
        };

        let mut query_strings: HashMap<String, String> = HashMap::new();

        if let Some(queries) = uri_iter.next() {
            for query in queries.split('&') {
                let mut query_iter = query.split('=');
                let key = match query_iter.next() {
                    Some(key) => key,
                    None => return Err(Error::ParsingError)
                };
                let value = match query_iter.next() {
                    Some(value) => value.trim(),
                    None => return Err(Error::ParsingError)
                };

                query_strings.insert(key.to_string(), value.to_string());
            }
        }

        Ok(Request { method, uri, version, headers, query_strings })
    }

}
