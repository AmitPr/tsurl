#[macro_use] extern crate rocket;
use rand::{Rng, distributions::Alphanumeric};

#[get("/")]
fn index() -> String {
    randomize()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

fn randomize() -> String {
    let extension: String = rand::thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
    return extension;
}