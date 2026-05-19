use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseDemographic {
    pub id: uuid::Uuid,
    pub key: String,
    pub demographic_type: String,
    pub label: Option<String>,
    pub lower_bound: Option<i32>,
    pub upper_bound: Option<i32>,
    pub registered: Option<bool>,
}
