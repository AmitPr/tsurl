use crate::database;
use rocket::serde::json::Json;
use rocket::State;

//TODO: Figure out how to use Catchers, Responders, etc correctly

#[get("/link/<url>")]
pub fn link_info(db: &State<database::DB>, url: String) -> Option<Json<database::URL>>{
    let url = db.urls.get(&url).unwrap();
    if url.is_none() {
        None
    } else {
        Some(Json(url.unwrap()))
    }
}

//Test this endpoint: curl -X PUT -H "Content-Type: application/json" -d '{"code":"abc","target":"https://abc.xyz"}' "127.0.0.1:8000/api/link"
#[put("/link", data ="<url>")]
pub fn create_link(db: &State<database::DB>, url: Json<database::URL>) -> Json<database::URL>{
    db.urls
        .insert(url.code.as_bytes(), url.clone())
        .unwrap();
    Json(url.0)
}

//Test this endpoint: curl -X DELETE "127.0.0.1:8000/api/link/<link>"
#[delete("/link/<url>")]
pub fn delete_link(db: &State<database::DB>, url: String) -> Option<Json<database::URL>>{
    let url = db.urls.remove(&url).unwrap();
    if url.is_none() {
        None
    } else {
        Some(Json(url.unwrap()))
    }
}