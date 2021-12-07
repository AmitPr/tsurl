#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;

use database::URL;
use frontend::frontend_utils::{FrontendError, LoadedAuth};

use rocket::{fs::FileServer, Build, Rocket, State};
use sled_extensions::DbExt;

mod api;
mod database;
mod frontend;

/// Catches all shortlinks and redirects to the original URL.
/// If the shortlink is not found, forwards the APIError status code.
#[get("/<code>")]
fn redir(db: &State<database::DB>, code: String) -> Result<URL, FrontendError> {
    let url = db.get_url(&code, true);
    match url {
        Ok(url) => Ok(url),
        Err(err) => Err(FrontendError::APIError(err)),
    }
}

/// Entrypoint for Rocket.
/// Initializes Database connection, and launches the server.
#[launch]
fn rocket() -> Rocket<Build> {
    let auth_file = "auth.txt";
    let auth = std::fs::read_to_string(auth_file).expect("Could not read auth file");
    let auth_object = LoadedAuth { credentials: auth };

    let db = sled_extensions::Config::default()
        .path("./sled_data")
        .open()
        .expect("Failed to open sled db");
    rocket::build()
        .mount("/static", FileServer::from("static"))
        .mount("/", FileServer::from("static/index").rank(30))
        .mount(
            "/admin",
            routes![
                frontend::admin_panel::authenticate,
                frontend::admin_panel::admin_dashboard
            ],
        )
        .attach(Template::fairing())
        .manage(database::DB {
            urls: db
                .open_bincode_tree("urls")
                .expect("Failed to open URL store"),
        })
        .manage(auth_object)
        .mount("/", routes![redir])
        .mount(
            "/api",
            routes![
                api::links::link_info,
                api::links::create_link,
                api::links::delete_link
            ],
        )
}

/*
fn randomize() -> String {
    let extension: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    return extension;
}
 */
