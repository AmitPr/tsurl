use rocket::{Request, http::Status, response::{Responder, content::Json}};
/// Error type for API responses. Possible errors are:
/// - `NotFound`: The requested resource was not found (404).
/// - `InternalServerError`: An internal server error occurred (500).
#[derive(Debug, Clone)]
pub enum APIError {
    NotFound,
    InternalServerError,
}

impl APIError {
    /// Returns a `rocket::http::Status` for this error.
    pub fn status(&self) -> Status {
        match self {
            APIError::NotFound => Status::NotFound,
            APIError::InternalServerError => Status::InternalServerError,
        }
    }
}

/// Formats an APIError into a JSON response.
impl<'r> Responder<'r, 'static> for APIError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json_string = match self {
            APIError::NotFound => format!("{{\"error\":{{\"code\":{},\"message\":\"{}\"}}}}", Status::NotFound.code, Status::NotFound.reason().unwrap()),
            APIError::InternalServerError => format!("{{\"error\":{{\"code\":{},\"message\":\"{}\"}}}}", Status::InternalServerError.code, Status::InternalServerError.reason().unwrap()),
        };
        Json(json_string).respond_to(&req)
    }
}
