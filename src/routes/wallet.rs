use actix_web::web;

use crate::models::country::list::Countries;
use crate::models::country::Country;

pub fn routes_handler() -> actix_web::Scope {
    web::scope("/wallets")
        .route("/", web::get().to(index))
        .route("/", web::post().to(new_tag))
}

/// extract path info using serde
async fn index() -> String {
    let count = Country {
        name: Countries::Nigeria,
        phone: String::from("+234"),
        abr: String::from("NGN"),
        state: vec![],
    };
    let serial = serde_json::to_string(&count).unwrap();
    format!("Get all  countries! {}", serial)
}

pub async fn new_tag() -> &'static str {
    "Create a new tag"
}
