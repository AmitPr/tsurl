use std::time::{SystemTime, UNIX_EPOCH};

use rocket::{
    http::Status,
    response::Responder,
    serde::{Deserialize, Serialize},
    Request, Response,
};
use sled_extensions::bincode::Tree;

use crate::api::api_utils::APIError;

/// Struct for storing database handles.
pub struct DB {
    pub urls: Tree<URL>,
}

impl DB {
    /// Fetch a URL object from the database, given the shortened URL code.
    pub fn get_url(&self, code: &String) -> Result<URL, APIError> {
        let url = self.urls.get(&code).unwrap();
        if url.is_none() {
            return Err(APIError::NotFound);
        } else {
            let url = url.unwrap();
            if let Some(expiration_time) = url.expiry_time {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                if now > expiration_time {
                    self.delete_link(&url.code)
                        .expect("Couldn't delete expired URL from DB");
                    return Err(APIError::NotFound);
                }
            }
            return Ok(url.clone());
        }
    }

    /// Delete a URL object from the database, given the shortened URL code.
    pub fn delete_link(&self, url: &String) -> Result<URL, APIError> {
        match self.urls.remove(&url) {
            Ok(None) => Err(APIError::NotFound),
            Ok(Some(url)) => Ok(url),
            Err(_) => Err(APIError::InternalServerError),
        }
    }
}

/// Struct for storing a URL object.
/// This is the format of the data stored in the database.
/// # Fields
/// * `code` - The shortened URL code.
/// * `url` - The original URL.
/// * `expiry_time` - The time at which the URL will expire, or `None` if it doesn't expire.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct URL {
    pub target: String,
    pub code: String,
    #[serde(default)]
    pub expiry_time: Option<u128>,
}

/// Sends a redirect response to the client, for the given URL object.
impl<'r> Responder<'r, 'static> for URL {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .status(Status::TemporaryRedirect)
            .raw_header("Location", self.target)
            .ok()
    }
}
