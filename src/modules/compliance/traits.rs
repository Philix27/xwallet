pub trait ICompliance {
    async fn provide_personal_details();
    async fn provide_bank_details();
    async fn compliance_status();
    async fn provide_documents();
}
