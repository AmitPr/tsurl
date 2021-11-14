#[macro_use]
extern crate rocket;
use rand::{distributions::Alphanumeric, Rng};
use rocket::{response::Redirect, Build, Rocket};
use sled_extensions::DbExt;

mod api;
mod database;

#[get("/test")]
fn redir() -> Redirect {
    Redirect::to(uri!("https://google.com"))
}

#[get("/")]
fn index() -> String {
    randomize()
}
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
