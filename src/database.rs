use std::{borrow::BorrowMut, time::{SystemTime, UNIX_EPOCH}};

use rocket::{Request, Response, http::Status, response::Responder, serde::{Deserialize, Serialize}};
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
            return Err(APIError::NotFound("URL not found.".to_string()));
        }
        let mut url = url.unwrap();
        let mut updated_url = url.clone();
        updated_url.num_hits += 1;
        self.insert_link(&updated_url);
        url = updated_url;
        if url.is_expired() {
            self.delete_link(&url.code)
                .expect("Couldn't delete expired URL from DB");
            return Err(APIError::NotFound("URL not found.".to_string()));
        }
        return Ok(url.clone());
    }

    /// Delete a URL object from the database, given the shortened URL code.
    pub fn delete_link(&self, url: &String) -> Result<URL, APIError> {
        match self.urls.remove(&url) {
            Ok(None) => Err(APIError::NotFound("URL not found.".to_string())),
            Ok(Some(url)) => Ok(url),
            Err(_) => Err(APIError::InternalServerError(
                "Couldn't delete URL from the database.".to_string(),
            )),
        }
    }

    pub fn insert_link(&self, url: &URL) -> Result<URL, APIError> {
        match self.urls.insert(url.code.as_bytes(), url.clone()) {
            Ok(_) => Ok(url.clone()),
            Err(_) => Err(APIError::InternalServerError(
                "Couldn't insert URL into the database.".to_string(),
            )),
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
    #[serde(default)]
    pub max_hits: Option<u64>,
    #[serde(skip)]
    pub num_hits : u64,
}

impl URL {
    pub fn is_expired(&self) -> bool {
        if let Some(expiration_time) = self.expiry_time {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
                if now > expiration_time {
                    return true;
                } else if let Some(max_hits) = self.max_hits {
                    if max_hits <= self.num_hits {
                        return true;
                    }
                }
        }
        
        return false;
    }
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
