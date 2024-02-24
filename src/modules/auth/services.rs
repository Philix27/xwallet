use actix_web::web::Json;

use crate::models::country::{list::Countries, Country};

use super::dtos;

pub struct AuthServices;

impl AuthServices {
    pub async fn index() -> String {
        format!("Auth Service")
    }
}

impl super::traits::IAuth for AuthServices {
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

    async fn verify_phone_otp() -> &'static str {
        "verify_phone_otp"
    }

    async fn send_phone_otp() -> &'static str {
        "send_phone_otp!"
    }

    async fn create_user() -> &'static str {
        "create_user!"
    }

    async fn verify_email_otp() -> &'static str {
        "verify_email_otp!"
    }

    async fn send_email_otp(body:  Json<dtos::SendEmailDto>) -> String {
        format!("send_email_otp!")
    }
}
