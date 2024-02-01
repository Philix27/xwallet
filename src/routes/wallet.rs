use actix_web::web;

pub fn routes_handler() -> actix_web::Scope {
    web::scope("/tags")
        .route("/", web::get().to(index))
        .route("/", web::post().to(new_tag))
}

/// extract path info using serde
async fn index() -> String {
    format!("Get all  !")
}

pub async fn new_tag() -> &'static str {
    "Create a new tag"
}
