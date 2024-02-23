// use super::services as auth;
use super::traits::ICompliance;
pub struct ComplianceRoutes;

impl ICompliance for ComplianceRoutes {
    async fn provide_personal_details() {
        todo!()
    }

    async fn provide_bank_details() {
        todo!()
    }

    async fn compliance_status() {
        todo!()
    }

    async fn provide_documents() {
        todo!()
    }
}
