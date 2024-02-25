use actix_web::Responder;

pub trait IPayment {
    async fn debit_card() -> impl Responder;
    async fn move_from_fiat_to_crypto() -> &'static str;
    async fn move_from_crypto_to_crypto() -> &'static str;
    async fn withdraw_fiat() -> &'static str;
    async fn send_crypto_to_external_wallet() -> &'static str;
    async fn send_fiat_to_external_bank_account() -> &'static str;
}
