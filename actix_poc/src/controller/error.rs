use actix_web::{error, http, HttpResponse};

#[derive(Fail, Debug)]
pub enum CustomError {
    #[fail(display = "internal error")]
    InternalError,
    #[fail(display = "invalid request {}", field)]
    InvalidInput { field: &'static str },
    #[fail(display = "timeout")]
    Timeout,
}

#[cfg_attr(tarpaulin, skip)]
impl error::ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::InternalError => {
                HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            CustomError::InvalidInput { .. } => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            CustomError::Timeout => HttpResponse::new(http::StatusCode::GATEWAY_TIMEOUT),
        }
    }
}
