pub trait IAuth {
    async fn index() -> String;
    async fn send_email_otp();
    async fn verify_email_otp();
    async fn send_phone_otp();
    async fn verify_phone_otp();
    async fn create_user();
}
