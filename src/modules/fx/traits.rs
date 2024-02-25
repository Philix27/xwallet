pub trait IFx {
    async fn provide_personal_details() -> &'static str;
    async fn provide_bank_details() -> &'static str;
    async fn compliance_status() -> &'static str;
    async fn provide_documents() -> &'static str;
}
