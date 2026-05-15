use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabasePoll {
    pub id: uuid::Uuid,
    pub source_id: uuid::Uuid,
    pub published_timestamp: NaiveDateTime,
}
