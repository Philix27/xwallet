use actix_web::web;

use super::services as auth;
use super::traits::IAuth;
pub struct AuthRoutes;

impl AuthRoutes {
    pub fn routes_handler() -> actix_web::Scope {
        web::scope("/auth")
            .route("/", web::post().to(Self::index))
            .route("/send_email_otp", web::post().to(Self::send_email_otp))
            .route("/verify_email_otp", web::post().to(Self::verify_email_otp))
            .route("/send_phone_otp", web::post().to(Self::send_phone_otp))
            .route("/verify_phone_otp", web::post().to(Self::verify_phone_otp))
            .route("/create_user", web::post().to(Self::create_user))
    }
}

impl IAuth for AuthRoutes {
    async fn index() -> String {
        auth::AuthServices::index().await
    }

    async fn send_email_otp() -> &'static str {
        "send_email_otp"
    }

    async fn verify_email_otp() -> &'static str {
        "verify_email_otp"
    }

    async fn send_phone_otp() -> &'static str {
        "send_phone_otp"
    }

    async fn verify_phone_otp() -> &'static str {
        "verify_phone_otp"
    }

    async fn create_user() -> &'static str {
        "create_user"
    }
}
