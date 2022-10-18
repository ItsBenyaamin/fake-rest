use std::{collections::HashMap, path::PathBuf};
use crate::{fake_rest::server_config::{Server, ServerDataSchema}, error};

use super::request::Request;

pub struct Status {
    pub code: usize,
    pub message: String
}

impl Status {
    pub fn ok() -> Self {
        Status { code: 200, message: String::from("OK") }
    }

    pub fn created() -> Self {
        Status { code: 201, message: String::from("Created") }
    }

    pub fn bad_request() -> Self {
        Status { code: 400, message: String::from("Bad Request") }
    }

    pub fn un_athorized() -> Self {
        Status { code: 401, message: String::from("Unauthorized") }
    }

    pub fn payment_required() -> Self {
        Status { code: 402, message: String::from("Payment Required") }
    }

    pub fn forbidden() -> Self {
        Status { code: 403, message: String::from("Forbidden") }
    }

    pub fn not_found() -> Self {
        Status { code: 404, message: String::from("Not Found") }
    }

    pub fn method_not_allowed() -> Self {
        Status { code: 405, message: String::from("Method Not Allowed") }
    }

    pub fn not_acceptable() -> Self {
        Status { code: 406, message: String::from("Not Acceptable") }
    }

    pub fn un_processable_entity() -> Self {
        Status { code: 422, message: String::from("Unprocessable Entity") }
    }

    pub fn internal_server_error() -> Self {
        Status { code: 500, message: String::from("Internal Server Error") }
    }

    pub fn from(status: usize) -> Self {
        match status {
            200 => Status::ok(),
            201 => Status::created(),
            400 => Status::bad_request(),
            401 => Status::un_athorized(),
            402 => Status::payment_required(),
            403 => Status::forbidden(),
            404 => Status::not_found(),
            405 => Status::method_not_allowed(),
            406 => Status::not_acceptable(),
            422 => Status::un_processable_entity(),
            500 => Status::internal_server_error(),
            _ => Status::ok()
        }
    }
}

pub struct Response {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl Response {

    pub async fn new(request: &Request, server: &Server) -> Result<Response, error::Error> {
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
                body: "Path not found".to_string()
            })
        }
        let server_data = server_data.unwrap();

        // check if method is same
        if request.method != server_data.method {
            return Ok(Response {
                status: Status::method_not_allowed(),
                headers: HashMap::new(),
                body: "Method Not Allowed".to_string()
            })
        }

        // check required headers
        if let Some(required_headers) = &server_data.headers {
            for header in required_headers.iter() {
                if !request.headers.contains_key(header) {
                    return Err(error::Error::ConfigRequiredHeadersError);
                }
            }
        }

        // check required query strings
        if let Some(required_query_strings) = &server_data.queries {
            for query in required_query_strings.iter() {
                if !request.query_strings.contains_key(query) {
                    return Err(error::Error::ConfigRequiredQueriesError);
                }
            }
        }

        // get body of request
        let body = if server_data.result_type == "direct" {
            server_data.result
        }else {
            let path = PathBuf::from(server_data.result);
            tokio::fs::read_to_string(path).await?
        };

        // get status of request
        let status = if let Some(status) = server_data.status_code {
            Status::from(status)
        }else {
            Status::ok()
        };

        // prepare response headers
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), body.len().to_string());
        if let Some(host) = request.headers.get("Host") {
            headers.insert("Host".to_string(), host.to_string());
        }
        if let Some(data_header) = &server_data.result_headers {
            for header_item in data_header.iter() {
                let mut item_iter = header_item.split(':');
                let key = match item_iter.next() {
                    Some(k) => k.trim(),
                    None => return Err(error::Error::ConfigParsingError),
                };
                let value = match item_iter.next() {
                    Some(v) => v.trim(),
                    None => return Err(error::Error::ConfigParsingError),
                };

                headers.insert(key.to_string(), value.to_string());
            }
        }
        Ok( Response { status, headers, body } )
    }

}