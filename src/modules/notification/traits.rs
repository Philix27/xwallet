pub trait INotification {
    async fn send_email_otp();
    async fn send_phone_otp();
    async fn email_msg();
    async fn phone_msg();
}
