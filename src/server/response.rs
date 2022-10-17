use std::collections::HashMap;

pub struct Status {
    pub code: usize,
    pub message: String
}

impl Status {
    pub fn OK() -> Self {
        Status { code: 200, message: String::from("OK") }
    }
}

pub struct Response {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl Response {

    pub fn new(status: Status, headers: HashMap<String, String>, body: String) -> Self {
        Response { status, headers, body }
    }

}