pub trait IFxRoutes {
    async fn get_currencies() -> &'static str;
    async fn get_exchanges() -> &'static str;
    async fn get_exchange_rates() -> &'static str;
}
