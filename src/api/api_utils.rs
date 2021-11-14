use rocket::{Request, http::Status, response::{Responder, content::Json}};

#[derive(Debug, Clone)]
pub enum APIError {
    NotFound,
    InternalServerError,
}

impl<'r> Responder<'r, 'static> for APIError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json_string = match self {
            APIError::NotFound => format!("{{\"error\":{{\"code\":{},\"message\":\"{}\"}}}}", Status::NotFound.code, Status::NotFound.reason().unwrap()),
            APIError::InternalServerError => format!("{{\"error\":{{\"code\":{},\"message\":\"{}\"}}}}", Status::InternalServerError.code, Status::InternalServerError.reason().unwrap()),
        };
        Json(json_string).respond_to(&req)
    }
}
