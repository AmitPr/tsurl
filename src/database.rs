use sled_extensions::bincode::Tree;
use rocket::serde::{Serialize, Deserialize};
pub struct DB {
    pub urls: Tree<URL>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct URL {
    pub target: String,
    pub code: String,
}
