use rocket::{
    http::Status,
    response::{content::Json, Responder},
    Request,
};
/// Error type for API responses. Possible errors are:
/// - `NotFound`: The requested resource was not found (404).
/// - `InternalServerError`: An internal server error occurred (500).
/// - `BadRequest`: The request was malformed (400).
#[derive(Debug, Clone)]
pub enum APIError {
    NotFound,
    InternalServerError,
    BadRequest,
}

impl APIError {
    /// Returns a `rocket::http::Status` for this error.
    pub fn status(&self) -> Status {
        match self {
            APIError::NotFound => Status::NotFound,
            APIError::InternalServerError => Status::InternalServerError,
            APIError::BadRequest => Status::BadRequest,
        }
    }
}

/// Formats an APIError into a JSON response.
impl<'r> Responder<'r, 'static> for APIError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json_string = match self {
            APIError::NotFound => format!(
                "{{\"error\":{{\"code\":{},\"message\":\"{}\"}}}}",
                Status::NotFound.code,
                Status::NotFound.reason().unwrap()
            ),
            APIError::InternalServerError => format!(
                "{{\"error\":{{\"code\":{},\"message\":\"{}\"}}}}",
                Status::InternalServerError.code,
                Status::InternalServerError.reason().unwrap()
            ),
            APIError::BadRequest => format!(
                "{{\"error\":{{\"code\":{},\"message\":\"{}\"}}}}",
                Status::BadRequest.code,
                Status::BadRequest.reason().unwrap()
            ),
        };
        Json(json_string).respond_to(&req)
    }
}
