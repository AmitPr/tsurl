#[macro_use] extern crate rocket;
use rocket::response::Redirect;

#[get("/test")]
fn redir() -> Redirect {
    Redirect::to(uri!("https://google.com"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![redir])
}