use rocket::{
    http::Status,
    response::Responder,
    serde::{Deserialize, Serialize},
    Request, Response,
};
use sled_extensions::bincode::Tree;

use crate::api::api_utils::APIError;
pub struct DB {
    pub urls: Tree<URL>,
}

impl DB {
    pub fn get_url(&self, code: &String) -> Result<URL, APIError> {
        let url = self.urls.get(&code).unwrap();
        if url.is_none() {
            return Err(APIError::NotFound);
        }
        Ok(url.unwrap().clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct URL {
    pub target: String,
    pub code: String,
}

impl<'r> Responder<'r, 'static> for URL {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .status(Status::TemporaryRedirect)
            .raw_header("Location", self.target)
            .ok()
    }
}
