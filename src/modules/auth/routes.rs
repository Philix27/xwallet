use super::services as auth;
use super::traits::IAuth;
pub struct AuthRoutes;

impl IAuth for AuthRoutes {
    async fn index() -> String {
        auth::AuthServices::index().await
    }

    async fn send_email_otp() {
        todo!()
    }

    async fn verify_email_otp() {
        todo!()
    }

    async fn send_phone_otp() {
        todo!()
    }

    async fn verify_phone_otp() {
        todo!()
    }

    async fn create_user() {
        todo!()
    }
}
