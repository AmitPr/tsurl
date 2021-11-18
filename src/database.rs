// use core::time;
// use std::time::{SystemTime, UNIX_EPOCH};
extern crate time;

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
//Scans database for redirect url when user tries using shortened link
impl DB {
    pub fn get_url(&self, code: &String) -> Result<URL, APIError> {
        let url = self.urls.get(&code).unwrap();
        if url.is_none() {
            return Err(APIError::NotFound);
        } else {
            let url_reference = url.as_ref().unwrap();
            if !url_reference.expiry_time.is_none() {
                let expiration_time = url_reference.expiry_time.unwrap();
                let system_time = time::get_time();
                let millisec = system_time.sec + system_time.nsec as i64 / 1000 / 1000;
                if expiration_time as i64 <= millisec {
                    self.delete_link(&url.unwrap().code);
                    return Err(APIError::NotFound);
                }
            }
        }
        Ok(url.unwrap().clone())
    }
    pub fn delete_link(&self, url: &String) -> Result<URL, APIError> {
        match self.urls.remove(&url) {
            Ok(None) => Err(APIError::NotFound),
            Ok(Some(url)) => Ok(url),
            Err(_) => Err(APIError::InternalServerError),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct URL {
    pub target: String,
    pub code: String,
    #[serde(default)]
    pub expiry_time: Option<u64>,
}


impl<'r> Responder<'r, 'static> for URL {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .status(Status::TemporaryRedirect)
            .raw_header("Location", self.target)
            .ok()
    }
}

