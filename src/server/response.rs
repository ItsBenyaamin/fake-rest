use std::{collections::HashMap, path::PathBuf};

use crate::{fake_rest::server_config::{Server, ServerDataSchema}, error::Error};
use crate::server::status::Status;
use super::{request::Request, content_type::ContentType, helpers};

pub struct Response {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

impl Response {

    pub async fn new(request: &Request, server: &Server) -> Result<Response, Error> {
        let mut headers = HashMap::new();

        let mut server_data: Option<ServerDataSchema> = None;
        for item in server.data.iter() {
            if item.path == request.uri {
                server_data = Some(item.clone());
                break;
            }
        }

        if server_data.is_none() {
            return Ok(Response {
                status: Status::not_found(),
                headers: HashMap::new(),
                body: "Path not found".as_bytes().to_vec()
            })
        }
        let server_data = server_data.unwrap();

        // check if method is same
        if request.method != server_data.method {
            return Ok(Response {
                status: Status::method_not_allowed(),
                headers: HashMap::new(),
                body: "Method Not Allowed".as_bytes().to_vec()
            })
        }

        // check required headers
        if let Some(required_headers) = &server_data.headers {
            for header in required_headers.iter() {
                let result = helpers::get_key_optional_value(header, ':')?;

                match request.headers.get(&result.0) {
                    Some(value) => {
                        if !result.1.is_empty() && value.as_str() != result.1 {
                            return Err(Error::ConfigRequiredHeadersError(
                                format!("the {} header's value is not equal.", header)
                            ));
                        }
                    }
                    None => return Err(Error::ConfigRequiredHeadersError(
                        format!("the `{}` header is not founded in the request.", header)
                    ))
                }
            }
        }

        // check required query strings
        if let Some(required_query_strings) = &server_data.queries {
            for query in required_query_strings.iter() {
                if !request.query_strings.contains_key(query) {
                    return Err(Error::ConfigRequiredQueriesError(
                        format!("the `{}` query is not founded in the request.", query)
                    ));
                }
            }
        }

        
        // get status of request
        let status = if let Some(status) = server_data.status_code {
            Status::from(status)
        }else {
            Status::ok()
        };

    
        // get body of request
        let body: Vec<u8> = match server_data.result_type.as_str() {
            "direct" => server_data.result.into_bytes(),
            "file" => {
                let path = PathBuf::from(&server_data.result);
                if !path.is_file() {
                    return Err(Error::ConfigFileOpenError(
                        format!("The given path is invalid or not a file: {}", &server_data.result)
                    ))
                }

                tokio::fs::read_to_string(path).await?.into_bytes()
            },
            "dl" => {
                let path = PathBuf::from(&server_data.result);
                if !path.is_file() {
                    return Err(Error::ConfigFileOpenError(
                        format!("The given path is invalid or not a file: {}", &server_data.result)
                    ))
                }

                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                let mut mime_type = String::new();
                match path.extension() {
                    Some(ext) => 
                        mime_type.push_str(
                            ContentType::get_mime_type(
                                ext.to_str().unwrap()
                            ).as_str()
                        ),
                    None => {},
                }
                    
                headers.insert("Content-Type".to_string(), mime_type);
                headers.insert("Accept-Ranges".to_string(), "None".to_string());
                headers.insert("Content-Disposition".to_string(), format!("attachment; filename={}", file_name));
                tokio::fs::read(path).await?
            },
            _ => Vec::new()
        };


        // prepare response headers
        headers.insert("Content-Length".to_string(), body.len().to_string());
        if let Some(host) = request.headers.get("Host") {
            headers.insert("Host".to_string(), host.to_string());
        }
        if let Some(data_header) = &server_data.result_headers {
            for header_item in data_header.iter() {
                let mut item_iter = header_item.split(':');
                let key = match item_iter.next() {
                    Some(k) => k.trim(),
                    None => return Err(Error::ConfigParsingError(header_item.to_string())),
                };
                let value = match item_iter.next() {
                    Some(v) => v.trim(),
                    None => return Err(Error::ConfigParsingError(header_item.to_string())),
                };

                headers.insert(key.to_string(), value.to_string());
            }
        }


        Ok( Response { status, headers, body } )
    }

}