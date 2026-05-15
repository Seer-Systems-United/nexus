#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseQuestion {
    pub id: uuid::Uuid,
    pub poll_id: uuid::Uuid,
    pub text: String,
    pub keywords: String,
}
