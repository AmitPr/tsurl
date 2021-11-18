use rocket::{
    response::Responder,
    Request,
};
use rocket_dyn_templates::Template;

use std::collections::BTreeMap;

use crate::api::api_utils::APIError;
/// A struct that mirrors APIError, but is used to generate an HTML error page.
pub enum FrontendError {
    APIError(APIError),
}

impl From<APIError> for FrontendError {
    /// Converts an APIError into a FrontendError.
    fn from(api_error: APIError) -> Self {
        FrontendError::APIError(api_error)
    }
}

/// Formats an APIError into a JSON response.
impl<'r> Responder<'r, 'static> for FrontendError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut data = BTreeMap::new();
        match self {
            FrontendError::APIError(err) => {
                data.insert("error_code", err.status().code.to_string());
                data.insert("error_message", err.message().clone());
            }
        };
        Template::render("error", &data).respond_to(&req)
    }
}
