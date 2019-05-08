use actix_web::{
    HttpResponse,
    error::ResponseError,
    http::StatusCode,
};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct ServiceError {
    pub http_status: StatusCode,
    pub message: String,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, messaage: &str) -> ServiceError {
        ServiceError {
            http_status,
            message: messaage.to_string(),
        }
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "status code: {}, message: {}", self.http_status.as_str(), self.message.as_str())
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).body(&self.message)
    }
}
