pub struct AuthServices;

impl AuthServices {
    pub async fn index() -> String {
        format!("Auth Service")
    }
}

impl super::traits::IAuth for AuthServices {
    async fn index() -> String {
        todo!()
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
