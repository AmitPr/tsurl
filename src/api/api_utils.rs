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
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
}

impl APIError {
    /// Returns a `rocket::http::Status` for this error.
    pub fn status(&self) -> Status {
        match self {
            APIError::NotFound(_) => Status::NotFound,
            APIError::InternalServerError(_) => Status::InternalServerError,
            APIError::BadRequest(_) => Status::BadRequest,
        }
    }
}

/// Formats an APIError into a JSON response.
impl<'r> Responder<'r, 'static> for APIError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json_string = match self {
            APIError::NotFound(message) => format!(
                "{{\"error\":{{\"code\":{},\"message\":\"{}\"}}}}",
                Status::NotFound.code,
                message
            ),
            APIError::InternalServerError(message) => format!(
                "{{\"error\":{{\"code\":{},\"message\":\"{}\"}}}}",
                Status::InternalServerError.code,
                message
            ),
            APIError::BadRequest(message) => format!(
                "{{\"error\":{{\"code\":{},\"message\":\"{}\"}}}}",
                Status::BadRequest.code,
                message
            ),
        };
        Json(json_string).respond_to(&req)
    }
}
