use crate::integration::termii;
use std::fmt::Error;

use super::email_svc::{self, EmailMsg, EmailPurpose};

pub struct NotificationServices;

impl NotificationServices {
    pub async fn index() -> String {
        format!("Notification Service")
    }
}

impl super::traits::INotification for NotificationServices {
    async fn index() -> String {
        todo!()
    }

    async fn send_email_otp() {
        todo!()
    }

    async fn send_phone_otp() {
        todo!()
    }

    async fn email_msg() {
        todo!()
    }

    async fn phone_msg() {
        todo!()
    }
}



impl termii::ITermii for NotificationServices {
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
impl email_svc::IEmailService for NotificationServices {
    fn send_mail(body: EmailMsg) -> () {
        match body.purpose {
            EmailPurpose::Newsletter => (),
            EmailPurpose::Transaction => (),
            EmailPurpose::Otp => (),
            EmailPurpose::Danger => (),
        }
    }
}
