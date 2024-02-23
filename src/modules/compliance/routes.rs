use actix_web::web;

use super::traits::ICompliance;
pub struct ComplianceRoutes;

impl ComplianceRoutes {
    pub fn routes_handler() -> actix_web::Scope {
        web::scope("/compliance")
            .route(
                "/provide_personal_details",
                web::post().to(Self::provide_personal_details),
            )
            .route(
                "/provide_bank_details",
                web::post().to(Self::provide_bank_details),
            )
            .route(
                "/compliance_status",
                web::post().to(Self::compliance_status),
            )
            .route(
                "/provide_documents",
                web::post().to(Self::provide_documents),
            )
    }
}

impl ICompliance for ComplianceRoutes {
    async fn provide_personal_details() -> &'static str {
        "provide_personal_details"
    }

    async fn provide_bank_details() -> &'static str {
        "provide_bank_details"
    }

    async fn compliance_status() -> &'static str {
        "compliance_status"
    }

    async fn provide_documents() -> &'static str {
        "provide_documents"
    }
}
