use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::models::country::list::Countries;
use crate::models::country::Country;
use crate::modules::AuthServices;

pub struct WalletRoutes;

impl WalletRoutes {
    pub fn routes_handler() -> actix_web::Scope {
        let middleware = HttpAuthentication::bearer(AuthServices::validator);
        web::scope("/wallets")
            // .wrap(middleware())
            .route("/", web::get().to(Self::index))
            .route("/", web::post().to(Self::new_tag))
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
}
