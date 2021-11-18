#[macro_use]
extern crate rocket;

use database::URL;
use rand::{distributions::Alphanumeric, Rng};
use rocket::{http::Status, Build, Rocket, State};
use sled_extensions::DbExt;

mod api;
mod database;

/// Catches all shortlinks and redirects to the original URL.
/// If the shortlink is not found, forwards the APIError status code.
#[get("/<code>")]
fn redir(db: &State<database::DB>, code: String) -> Result<URL, Status> {
    let url = db.get_url(&code);
    match url {
        Ok(url) => Ok(url),
        Err(err) => Err(err.status()),
    }
}

/// Landing page for the service.
#[get("/")]
fn index() -> String {
    randomize()
}

/// Entrypoint for Rocket.
/// Initializes Database connection, and launches the server.
#[launch]
fn rocket() -> Rocket<Build> {
    let db = sled_extensions::Config::default()
        .path("./sled_data")
        .open()
        .expect("Failed to open sled db");
    rocket::build()
        .manage(database::DB {
            urls: db
                .open_bincode_tree("urls")
                .expect("Failed to open URL store"),
        })
        .mount("/", routes![index, redir])
        .mount(
            "/api",
            routes![
                api::links::link_info,
                api::links::create_link,
                api::links::delete_link
            ],
        )
}

fn randomize() -> String {
    let extension: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    return extension;
}
