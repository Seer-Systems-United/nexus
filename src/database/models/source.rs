use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseSource {
    pub id: uuid::Uuid,
    pub name: String,
}
