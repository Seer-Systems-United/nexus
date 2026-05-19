use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseResponseUnit {
    pub id: uuid::Uuid,
    pub name: String,
}
