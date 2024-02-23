use super::traits::ICompliance;

pub struct ComplianceServices;

impl ICompliance for ComplianceServices {
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
