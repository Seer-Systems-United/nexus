use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationInput {
    pub question_fingerprint: String,
    pub source: String,
    pub poll_date: Option<String>,
    pub source_collection: String,
    pub question_title: String,
    pub prompt: String,
    pub answer_labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationOutput {
    #[serde(default)]
    pub canonical_topic_id: String,
    #[serde(default)]
    pub canonical_label: String,
    #[serde(default)]
    pub intent: String,
    #[serde(default)]
    pub subject: Vec<String>,
    #[serde(default)]
    pub confidence: f32,
    #[serde(default)]
    pub exclude_reason: Option<String>,
}
