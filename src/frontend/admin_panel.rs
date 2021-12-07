use std::collections::BTreeMap;

use htpasswd_verify::load;
use rocket::{form::Form, State, http::Cookie, http::CookieJar, response::Redirect};
use rocket_dyn_templates::Template;

use crate::{
    api::api_utils::APIError,
    frontend::frontend_utils::{AuthObject, FrontendError, LoadedAuth}, database,
};
#[post("/auth", data = "<req_auth>")]
pub fn authenticate(
    auth: &State<LoadedAuth>,
    req_auth: Form<AuthObject>,
    cookies: &CookieJar<'_>
) -> Result<Redirect, FrontendError> {
    if load(auth.credentials.as_str()).check(&req_auth.username, &req_auth.password) {
        cookies.add_private(Cookie::new("auth", "true"));
        Ok(Redirect::to("/admin"))
    } else {
        Err(FrontendError::APIError(APIError::Unauthorized(
            "Invalid username or password".to_string(),
        )))
    }
}

#[get("/")]
pub fn admin_dashboard(db: &State<database::DB>, cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    if cookies.get_private("auth").is_some() {
        let mut data = BTreeMap::new();
        data.insert("urls", db.get_all_urls());
        Ok(Template::render("admin_dashboard", &data))
    } else {
        Err(Redirect::to("/"))
    }
}