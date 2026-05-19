use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseQuestion {
    pub id: uuid::Uuid,
    pub poll_id: uuid::Uuid,
    pub text: String,
    pub keywords: String,
}
