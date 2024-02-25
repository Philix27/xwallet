// use std::error::Error;

use actix_web::{dev::ServiceRequest, error::Error, web::Json, HttpMessage};
use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use hmac::{digest::KeyInit, Hmac};
use jwt::VerifyWithKey;
use sha2::Sha256;

use crate::{
    config::AppEnv,
    models::country::{list::Countries, Country},
};

use super::dtos;

pub struct AuthServices;

impl AuthServices {
    pub async fn index() -> String {
        format!("Auth Service")
    }
    pub fn middleware() {
        // pub fn middleware() -> HttpAuthentication<BearerAuth, ()> {
        // let c = HttpAuthentication::bearer(Self::validator);
        HttpAuthentication::bearer(Self::validator);
    }
    pub async fn validator(
        req: ServiceRequest,
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, (Error, ServiceRequest)> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(AppEnv::new().jwt_secret.as_bytes()).unwrap();
        let token_string = credentials.token();

        let claims: Result<dtos::TokenClaims, &str> = token_string
            .verify_with_key(&key)
            .map_err(|_| "Invalid token");

        match claims {
            Ok(value) => {
                req.extensions_mut().insert(value);
                Ok(req)
            }
            Err(_) => {
                let config = req
                    .app_data::<bearer::Config>()
                    .cloned()
                    .unwrap_or_default()
                    .scope("");

                Err((AuthenticationError::from(config).into(), req))
            }
        }
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

    async fn send_email_otp(_body: Json<dtos::SendEmailDto>) -> String {
        format!("send_email_otp!")
    }
}
