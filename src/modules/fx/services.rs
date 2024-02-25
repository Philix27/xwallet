use super::traits::IFxRoutes;

pub struct FxService;

impl IFxRoutes for FxService {
    async fn get_currencies() -> &'static str {
        "get_currencies"
    }

    async fn get_exchanges() -> &'static str {
        "get_exchanges"
    }

    async fn get_exchange_rates() -> &'static str {
        "get_exchange_rates"
    }
}
