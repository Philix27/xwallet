use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageBody {
    pub api_key: String,
    pub to: String,
    pub from: String,
    pub sms: String,
    /// type = plain
    #[serde(rename = "type")]
    pub types: String,
    pub channel: Channel,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Channel {
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(rename = "dnd")]
    Dnd,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageResponse {
    pub message_id: String,
    pub message: String,
    pub balance: u64,
    pub user: String,
}
