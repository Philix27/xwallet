use crate::integration::termii;
use std::fmt::Error;
trait ISendMsgPhoneNo {
    async fn send_msg_to_phone(body: termii::PhoneBody) -> Result<termii::PhoneResponse, Error>;
}

pub struct NotificationService {}

impl ISendMsgPhoneNo for NotificationService {
    async fn send_msg_to_phone(body: termii::PhoneBody) -> Result<termii::PhoneResponse, Error> {
        Ok(termii::PhoneResponse {
            message_id: todo!(),
            message: todo!(),
            balance: todo!(),
            user: todo!(),
        })
    }
}
