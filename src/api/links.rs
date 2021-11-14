use crate::database;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::api_utils::APIError;

#[get("/link/<url>")]
pub fn link_info(db: &State<database::DB>, url: String) -> Result<Json<database::URL>, APIError> {
    let url = db.urls.get(&url).unwrap();
    if url.is_none() {
        Err(APIError::NotFound)
    } else {
        Ok(Json(url.unwrap()))
    }
}

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

//Test this endpoint: curl -X DELETE "127.0.0.1:8000/api/link/<link>"
#[delete("/link/<url>")]
pub fn delete_link(db: &State<database::DB>, url: String) -> Result<Json<database::URL>, APIError> {
    match db.urls.remove(&url) {
        Ok(None) => Err(APIError::NotFound),
        Ok(Some(url)) => Ok(Json(url)),
        Err(_) => Err(APIError::InternalServerError),
    }
}
