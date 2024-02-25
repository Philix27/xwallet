use crate::integration::termii;
use std::fmt::Error;

use super::email_svc::{EmailMsg, EmailPurpose};

pub struct NotificationServices;

impl NotificationServices {
    async fn msg_phone(body: termii::PhoneBody) -> Result<termii::PhoneResponse, Error> {
        Ok(termii::PhoneResponse {
            message_id: body.sms,
            message: body.from,
            balance: 23,
            user: body.to,
        })
    }
    async fn bulk_msg_to_phone() -> String {
        String::from("Not yet implmeneted")
    }
    fn msg_email(body: EmailMsg) -> () {
        match body.purpose {
            EmailPurpose::Newsletter => (),
            EmailPurpose::Transaction => (),
            EmailPurpose::Otp => (),
            EmailPurpose::Danger => (),
        }
    }
}

impl super::traits::INotification for NotificationServices {
    async fn send_email_otp() {
        // Self::msg_phone(body);
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
