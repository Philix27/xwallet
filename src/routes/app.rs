use actix_web::web;

pub async fn index() -> String {
    format!("Welcome to XWallet Sever")
}
