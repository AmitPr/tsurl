#[macro_use]
extern crate rocket;
extern crate time;

use database::URL;
use rand::{distributions::Alphanumeric, Rng};
use rocket::{http::Status, Build, Rocket, State};
use sled_extensions::DbExt;

mod api;
mod database;

#[get("/<code>")]
fn redir(db: &State<database::DB>, code: String) -> Result<URL, Status> {
    let url = db.get_url(&code);
    let time;
    if !url.is_err() {
        time = url.as_ref().unwrap().expiry_time.unwrap();
        let system_time = time::get_time();
        let millisec = system_time.sec + system_time.nsec as i64 / 1000 / 1000;
        if time as i64 >= millisec {
            panic!("Time has expired")
        }
    }

    match url {
        Ok(url) => Ok(url),
        Err(err) => Err(err.status()),
    }
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
