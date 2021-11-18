use crate::database;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::api_utils::APIError;

/// API endpoint for fetching URL information.
/// Request should be sent to `/api/links/<url>`, Where `<url>` is the shortened URL code.
#[get("/link/<url>")]
pub fn link_info(db: &State<database::DB>, url: String) -> Result<Json<database::URL>, APIError> {
    db.get_url(&url).map(|url| Json(url))
}

/// API endpoint for creating a new URL.
/// Request body should be a JSON object with the following fields:
/// - url: the URL to shorten
/// - code: the code to use for the shortened URL
/// - expiry_time: the time at which the shortened URL should expire, as a UNIX timestamp in milliseconds (optional)
///
/// # Example Requests
/// ```json
/// {
///     "code":"short",
///     "target":"https://example.com",
/// }
/// ```
/// With Expiry Time:
/// ```json
/// {
///     "code":"short",
///     "target":"https://example.com",
///     "expiry_time":1568888888
/// }
/// ```
//Test this endpoint: curl -X PUT -H "Content-Type: application/json" -d '{"code":"abc","target":"https://abc.xyz"}' "127.0.0.1:8000/api/link"
#[put("/link", data = "<url>")]
pub fn create_link(
    db: &State<database::DB>,
    url: Json<database::URL>,
) -> Result<Json<database::URL>, APIError> {
    match db.urls.insert(url.code.as_bytes(), url.clone()) {
        Ok(_) => Ok(Json(url.0)),
        Err(_) => Err(APIError::InternalServerError),
    }
}

/// API endpoint for deleting a URL.
/// Request should be sent to `/api/links/<url>`, Where `<url>` is the shortened URL code.
//Test this endpoint: curl -X DELETE "127.0.0.1:8000/api/link/<link>"
#[delete("/link/<url>")]
pub fn delete_link(db: &State<database::DB>, url: String) -> Result<Json<database::URL>, APIError> {
    match db.urls.remove(&url) {
        Ok(None) => Err(APIError::NotFound),
        Ok(Some(url)) => Ok(Json(url)),
        Err(_) => Err(APIError::InternalServerError),
    }
}
