use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabasePerson {
    pub id: uuid::Uuid,
    pub given_name: String,
    pub surname: String,
    pub suffix: Option<String>,
    pub prefix: Option<String>,
}
