use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabasePoll {
    pub id: uuid::Uuid,
    pub source_id: uuid::Uuid,
    pub published_timestamp: NaiveDateTime,
}
