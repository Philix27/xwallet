use actix_web::web;

use super::traits::IFxRoutes;
pub struct FxRoutes;

impl FxRoutes {
    pub fn routes_handler() -> actix_web::Scope {
        web::scope("/compliance")
            .route(
                "/provide_personal_details",
                web::get().to(Self::get_currencies),
            )
            .route(
                "/provide_bank_details",
                web::get().to(Self::get_exchange_rates),
            )
            .route("/compliance_status", web::get().to(Self::get_exchanges))
    }
}

impl IFxRoutes for FxRoutes {
    async fn get_currencies() -> &'static str {
        "provide_personal_details"
    }

    async fn get_exchanges() -> &'static str {
        "provide_personal_details"
    }

    async fn get_exchange_rates() -> &'static str {
        "provide_personal_details"
    }
}
