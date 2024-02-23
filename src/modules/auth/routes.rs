pub struct AuthRoutes;

use super::traits::IAuth;
use super::services as auth;

impl AuthRoutes {
    pub async fn index() -> String {
        format!("Welcome to XWallet Sever")
    }
}

impl IAuth for AuthRoutes {
    async fn index() -> String {
        auth::AuthServices::index().await
    }

    async fn send_email_otp() {
        todo!()
    }

    async fn send_phone_otp() {
        todo!()
    }

    async fn create_user() {
        todo!()
    }
}
