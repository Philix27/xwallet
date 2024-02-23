use actix_web::web;

use super::services as service;
use super::traits::ITransactions;
pub struct TransactionsRoutes;

impl TransactionsRoutes {
    pub fn routes_handler() -> actix_web::Scope {
        web::scope("/transactions")
            .route("/", web::post().to(Self::index))
            .route(
                "/get_last_month_transactions",
                web::get().to(Self::get_last_month_transactions),
            )
            .route(
                "/get_last_six_month_transactions",
                web::get().to(Self::get_last_six_month_transactions),
            )
            .route(
                "/get_transactions_for",
                web::get().to(Self::get_transactions_for),
            )
    }
}

impl ITransactions for TransactionsRoutes {
    async fn index() -> String {
        service::TransactionsServices::index().await
    }

    async fn get_last_month_transactions() -> &'static str {
        "get_last_month_transactions"
    }

    async fn get_last_six_month_transactions() -> &'static str {
        "get_last_six_month_transactions"
    }

    async fn get_transactions_for() -> &'static str {
        "get_transactions_for"
    }
}
