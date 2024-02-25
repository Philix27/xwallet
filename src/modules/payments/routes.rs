use actix_web::web;
use super::traits::IPayment;
pub struct PaymentRoutes;

impl PaymentRoutes {
    pub fn routes_handler() -> actix_web::Scope {
        web::scope("/debit_card")
            .route(
                "/provide_personal_details",
                web::get().to(Self::debit_card),
            )
            .route(
                "/move_from_crypto_to_crypto",
                web::get().to(Self::move_from_crypto_to_crypto),
            )
            .route(
                "/move_from_fiat_to_crypto",
                web::get().to(Self::move_from_fiat_to_crypto),
            )
            .route("/withdraw_fiat", web::post().to(Self::withdraw_fiat))
    }
}

impl IPayment for PaymentRoutes {
    async fn debit_card() -> &'static str {
        todo!()
    }

    async fn move_from_fiat_to_crypto() -> &'static str {
        todo!()
    }

    async fn move_from_crypto_to_crypto() -> &'static str {
        todo!()
    }

    async fn withdraw_fiat() -> &'static str {
        todo!()
    }

    async fn send_crypto_to_external_wallet() -> &'static str {
        todo!()
    }

    async fn send_fiat_to_external_bank_account() -> &'static str {
        todo!()
    }
}
