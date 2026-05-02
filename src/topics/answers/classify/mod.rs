//! # Answer classification module
//!
//! Classifies answer labels into canonical forms.
//! Handles approval, favorability, direction, and ballot answers.

mod approval;
mod ballot;

use crate::topics::answers::{
    answer, generic_answer,
    support::{models::MappedAnswer, text},
};
use crate::topics::catalog::{
    ECONOMY_APPROVAL_ID, FOREIGN_POLICY_APPROVAL_ID, GENERIC_BALLOT_ID, IMMIGRATION_APPROVAL_ID,
    INFLATION_APPROVAL_ID, PRESIDENTIAL_APPROVAL_ID, RIGHT_DIRECTION_ID, TRUMP_FAVORABILITY_ID,
};

/// Map an answer label to a canonical answer based on topic ID.
///
/// # Parameters
/// - `topic_id`: The topic ID for topic-specific handling.
/// - `label`: The raw answer label to classify.
///
/// # Returns
/// - `MappedAnswer` with canonical ID, label, and priority.
pub fn map_answer(topic_id: &str, label: &str) -> MappedAnswer {
    let normalized = text::normalize_text(label);
    let lower = normalized.to_ascii_lowercase();

    if is_approval_topic(topic_id)
        && let Some(answer) = approval::approval_answer(&normalized)
    {
        return answer;
    }
    if topic_id == TRUMP_FAVORABILITY_ID
        && let Some(answer) = approval::favorability_answer(&normalized)
    {
        return answer;
    }
    if topic_id == RIGHT_DIRECTION_ID
        && let Some(answer) = ballot::direction_answer(&normalized)
    {
        return answer;
    }
    if topic_id == GENERIC_BALLOT_ID
        && let Some(answer) = ballot::ballot_answer(&lower)
    {
        return answer;
    }

    generic_answer(topic_id, &normalized)
}

/// Check if a topic ID is an approval topic.
fn is_approval_topic(topic_id: &str) -> bool {
    matches!(
        topic_id,
        PRESIDENTIAL_APPROVAL_ID
            | ECONOMY_APPROVAL_ID
            | INFLATION_APPROVAL_ID
            | IMMIGRATION_APPROVAL_ID
            | FOREIGN_POLICY_APPROVAL_ID
    )
}

/// Check if an answer is "unsure" type.
///
/// # Parameters
/// - `label`: The normalized answer label.
///
/// # Returns
/// - `Some(MappedAnswer)` if it matches unsure patterns.
pub fn unsure_answer(label: &str) -> Option<MappedAnswer> {
    let lower = label.to_ascii_lowercase();
    if lower.contains("don't know")
        || lower.contains("dont know")
        || lower.contains("not sure")
        || lower.contains("unsure")
    {
        return Some(answer("unsure", "Unsure", 3));
    }
    if lower.contains("skipped") {
        return Some(answer("skipped", "Skipped", 3));
    }

    None
}
