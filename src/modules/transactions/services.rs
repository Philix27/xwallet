use crate::models::country::{list::Countries, Country};

pub struct TransactionsServices;

impl TransactionsServices {
    pub async fn index() -> String {
        format!("Auth Service")
    }
}

impl super::traits::ITransactions for TransactionsServices {
    async fn index() -> String {
        let count = Country {
            name: Countries::Nigeria,
            phone: String::from("+234"),
            abr: String::from("NGN"),
            state: vec![],
        };
        let serial = serde_json::to_string(&count).unwrap();
        format!("Get all  countries! {}", serial)
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
