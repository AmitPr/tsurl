#[macro_use] extern crate rocket;
use rocket::response::Redirect;
use rand::{Rng, distributions::Alphanumeric};

#[get("/test")]
fn redir() -> Redirect {
    Redirect::to(uri!("https://google.com"))
}

#[get("/")]
fn index() -> String {
    randomize()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,redir])
}

fn randomize() -> String {
    let extension: String = rand::thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
    return extension;
}