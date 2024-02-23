use super::traits::ICompliance;

pub struct ComplianceServices;

impl ICompliance for ComplianceServices {
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
