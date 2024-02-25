use super::traits::IPayment;

pub struct PaymentService;

impl IPayment for PaymentService {
    async fn debit_card() -> &'static str {
       "opposite"
    }

    async fn move_from_fiat_to_crypto() -> &'static str {
       "opposite"
    }

    async fn move_from_crypto_to_crypto() -> &'static str {
       "opposite"
    }

    async fn withdraw_fiat() -> &'static str {
       "opposite"
    }

    async fn send_crypto_to_external_wallet() -> &'static str {
       "opposite"
    }

    async fn send_fiat_to_external_bank_account() -> &'static str {
       "opposite"
    }
}
