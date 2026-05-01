use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum SourceId {
    Emerson,
    Gallup,
    Ipsos,
    YouGov,
}

impl SourceId {
    pub const ALL: [Self; 4] = [Self::Emerson, Self::Gallup, Self::Ipsos, Self::YouGov];

    pub fn id(self) -> &'static str {
        match self {
            Self::Emerson => "emerson",
            Self::Gallup => "gallup",
            Self::Ipsos => "ipsos",
            Self::YouGov => "yougov",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Emerson => "Emerson",
            Self::Gallup => "Gallup",
            Self::Ipsos => "Ipsos",
            Self::YouGov => "YouGov",
        }
    }
}

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
pub struct TopicSource {
    pub id: String,
    pub name: String,
}

impl From<SourceId> for TopicSource {
    fn from(source: SourceId) -> Self {
        Self {
            id: source.id().to_string(),
            name: source.name().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DemographicGroup {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DemographicValue {
    pub id: String,
    pub label: String,
    pub group: Option<DemographicGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnswerResult {
    pub id: String,
    pub label: String,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DemographicResult {
    pub demographic: DemographicValue,
    pub answers: Vec<AnswerResult>,
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
    pub compatibility: Compatibility,
    pub demographics: Vec<DemographicResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PooledAnswerResult {
    pub id: String,
    pub label: String,
    pub value: f32,
    pub observation_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PooledDemographicResult {
    pub demographic: DemographicValue,
    pub answers: Vec<PooledAnswerResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TopicCollection {
    pub topic: TopicSummary,
    pub scope: crate::sources::Scope,
    pub observations: Vec<TopicObservation>,
    pub pooled: Vec<PooledDemographicResult>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HeadlineTopicSummary {
    pub topic: TopicSummary,
    pub observation_count: usize,
    pub source_count: usize,
    pub sources: Vec<TopicSource>,
    pub latest_date: Option<String>,
    pub sample_questions: Vec<String>,
}
