pub trait ITransactions {
    async fn index() -> String;
    async fn get_last_month_transactions() -> &'static str;
    async fn get_last_six_month_transactions() -> &'static str;
    async fn get_transactions_for() -> &'static str;
}
