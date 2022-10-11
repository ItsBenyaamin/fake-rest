use std::collections::HashMap;


pub struct Request {
    path: String,
    headers: HashMap<String, String>
}

impl Request {

    pub fn empty() -> Self {
        Request { path: "".to_string(), headers: HashMap::new() }
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

}

pub fn parse_request() -> Request {
    let request = Request::empty();
    unimplemented!()
}