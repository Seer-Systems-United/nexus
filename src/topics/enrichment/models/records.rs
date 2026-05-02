//! # Enrichment record models
//!
//! Data structures for the question enrichment index.
//! Stores classified questions with metadata and review status.

use super::{ClassificationInput, ClassificationOutput};
use crate::topics::enrichment::{INDEX_VERSION, MIN_APPLY_CONFIDENCE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionIndex {
    #[serde(default = "default_index_version")]
    pub version: u32,
    #[serde(default)]
    pub records: Vec<QuestionEnrichment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionEnrichment {
    pub question_fingerprint: String,
    pub source: String,
    pub poll_date: Option<String>,
    pub source_collection: String,
    pub question_title: String,
    pub prompt: String,
    #[serde(default)]
    pub answer_labels: Vec<String>,
    pub canonical_topic_id: String,
    pub canonical_label: String,
    pub intent: String,
    #[serde(default)]
    pub subject: Vec<String>,
    pub confidence: f32,
    pub model: String,
    pub review_status: String,
    pub exclude_reason: Option<String>,
}

impl QuestionEnrichment {
    pub(in crate::topics::enrichment) fn from_classification(
        input: ClassificationInput,
        output: ClassificationOutput,
        model: &str,
    ) -> Self {
        let review_status = if output.exclude_reason.is_some() {
            "excluded"
        } else if output.confidence < MIN_APPLY_CONFIDENCE {
            "needs-review"
        } else {
            "accepted"
        };

        Self {
            question_fingerprint: input.question_fingerprint,
            source: input.source,
            poll_date: input.poll_date,
            source_collection: input.source_collection,
            question_title: input.question_title,
            prompt: input.prompt,
            answer_labels: input.answer_labels,
            canonical_topic_id: output.canonical_topic_id,
            canonical_label: output.canonical_label,
            intent: output.intent,
            subject: output.subject,
            confidence: output.confidence,
            model: model.to_string(),
            review_status: review_status.to_string(),
            exclude_reason: output.exclude_reason,
        }
    }
}

impl Default for QuestionIndex {
    fn default() -> Self {
        Self {
            version: INDEX_VERSION,
            records: Vec::new(),
        }
    }
}

fn default_index_version() -> u32 {
    INDEX_VERSION
}
