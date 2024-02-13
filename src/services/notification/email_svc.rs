use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EmailPurpose {
    Otp,
    Newsletter,
    Transaction,
    Danger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailMsg {
    pub to: String,
    pub purpose: EmailPurpose,
    pub msg: String,
}
pub trait IEmailService {
    fn send_mail(body: EmailMsg) -> ();
}
