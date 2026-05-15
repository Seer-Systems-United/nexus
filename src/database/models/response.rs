#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseResponse {
    pub id: uuid::Uuid,
    pub question_id: uuid::Uuid,
    pub demographic_id: uuid::Uuid,
    pub unit_id: uuid::Uuid,
    pub answer: String,
    pub value: i32,
}
