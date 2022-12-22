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