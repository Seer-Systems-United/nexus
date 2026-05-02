use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TopicStatus {
    Stable,
    Headline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Compatibility {
    ExactWording,
    EquivalentWording,
    RollupCompatible,
    TrendComparable,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TopicSummary {
    pub id: String,
    pub label: String,
    pub status: TopicStatus,
    pub description: Option<String>,
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TopicCollection {
    pub topic: TopicSummary,
    pub scope: crate::sources::Scope,
    pub observations: Vec<super::TopicObservation>,
    pub pooled: Vec<super::PooledDemographicResult>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HeadlineTopicSummary {
    pub topic: TopicSummary,
    pub observation_count: usize,
    pub source_count: usize,
    pub sources: Vec<super::TopicSource>,
    pub latest_date: Option<String>,
    pub sample_questions: Vec<String>,
}
