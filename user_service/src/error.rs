use actix_web::{error, http, HttpResponse};

#[derive(Fail, PartialEq, Debug)]
pub enum CustomError {
    #[fail(display = "internal error {}", field)]
    InternalError { field: &'static str },
    #[fail(display = "invalid request {}", field)]
    InvalidInput { field: &'static str },
    #[fail(display = "timeout")]
    Timeout,
    #[fail(display = "validation error: {}", field)]
    ValidationError { field: &'static str },
}

impl error::ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::InternalError { .. } => {
                HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            CustomError::InvalidInput { .. } => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            CustomError::Timeout => HttpResponse::new(http::StatusCode::GATEWAY_TIMEOUT),
            CustomError::ValidationError { .. } => HttpResponse::new(http::StatusCode::BAD_REQUEST),
        }
    }
}
