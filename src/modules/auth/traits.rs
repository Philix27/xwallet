pub trait IAuth {
    async fn index() -> String;
    async fn send_email_otp() -> &'static str;
    async fn verify_email_otp() -> &'static str;
    async fn send_phone_otp() -> &'static str;
    async fn verify_phone_otp() -> &'static str;
    async fn create_user() -> &'static str;
}
