use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub struct SendEmailDto {
    pub msg: String,
    pub user_id: String,
}

impl Display for SendEmailDto {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //    Ok( format!("msg: {}, user_id: {}", self.msg, self.user_id))
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: i32,
}
