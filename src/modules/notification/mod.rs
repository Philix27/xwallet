mod email_svc;

use crate::integration::termii;
use email_svc::{EmailMsg, EmailPurpose};
use std::fmt::Error;
pub struct NotificationService {}

impl termii::ITermii for NotificationService {
    async fn send_msg_to_phone(body: termii::PhoneBody) -> Result<termii::PhoneResponse, Error> {
        Ok(termii::PhoneResponse {
            message_id: todo!(),
            message: todo!(),
            balance: todo!(),
            user: todo!(),
        })
    }
    async fn send_bulk_msg_to_phone() -> String {
        String::from("Not yet implmeneted")
    }
}
impl email_svc::IEmailService for NotificationService {
    fn send_mail(body: EmailMsg) -> () {
        match body.purpose {
            EmailPurpose::Newsletter => (),
            EmailPurpose::Transaction => (),
            EmailPurpose::Otp => (),
            EmailPurpose::Danger => (),
        }
    }
}

