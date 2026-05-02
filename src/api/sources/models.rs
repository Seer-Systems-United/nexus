use crate::sources::SourceId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SourceSummary {
    pub id: &'static str,
    pub name: &'static str,
}

#[derive(Debug, Deserialize)]
pub struct SourceQuery {
    pub scope: Option<String>,
    pub count: Option<u32>,
    pub n: Option<u32>,
    pub question: Option<String>,
}

impl From<SourceId> for SourceSummary {
    fn from(source: SourceId) -> Self {
        Self {
            id: source.id(),
            name: source.name(),
        }
    }
}
