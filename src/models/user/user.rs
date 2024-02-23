use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::country::Country;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub country: Country,
    pub status: UserStatus,
    pub verification_status: UserVerificationStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum UserStatus {
    SignUp,
    CompletedKyc,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum UserVerificationStatus {
    NotVerified,
    Verified,
    UnderReview,
}

