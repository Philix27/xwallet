use crate::models::user::CreateUserInput;

pub async fn index() -> String {
    format!("Welcome to XWallet Sever")
}

pub async fn create_user(body: CreateUserInput) -> String {
    format!("Welcome to XWallet Sever")
}
