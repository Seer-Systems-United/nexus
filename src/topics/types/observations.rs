//! # Observation types
//!
//! Data types for polling observations and sources.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TopicSource {
    pub id: String,
    pub name: String,
}

impl From<crate::sources::SourceId> for TopicSource {
    fn from(source: crate::sources::SourceId) -> Self {
        Self {
            id: source.id().to_string(),
            name: source.name().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TopicObservation {
    pub id: String,
    pub topic_id: String,
    pub topic_label: String,
    pub source: TopicSource,
    pub source_collection: String,
    pub source_subtitle: Option<String>,
    pub question_title: String,
    pub prompt: String,
    pub poll_date: Option<String>,
    pub compatibility: super::Compatibility,
    pub demographics: Vec<super::DemographicResult>,
}
