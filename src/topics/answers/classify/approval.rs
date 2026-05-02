//! # Approval answer classifier
//!
//! Maps answer labels to "Approve"/"Disapprove" or
//! "Favorable"/"Unfavorable" canonical answers.

use crate::topics::answers::{
    answer,
    support::{models::MappedAnswer, text},
};

/// Map an answer label to an approval answer.
///
/// # Parameters
/// - `label`: The normalized answer label.
///
/// # Returns
/// - `Some(MappedAnswer)` if it matches approval patterns.
pub fn approval_answer(label: &str) -> Option<MappedAnswer> {
    let lower = label.to_ascii_lowercase();
    if lower.contains("disapprove") {
        return Some(answer(
            "disapprove",
            "Disapprove",
            net_priority(&lower, "disapprove"),
        ));
    }
    if lower.contains("approve") {
        return Some(answer(
            "approve",
            "Approve",
            net_priority(&lower, "approve"),
        ));
    }
    super::unsure_answer(label)
}

/// Map an answer label to a favorability answer.
///
/// # Parameters
/// - `label`: The normalized answer label.
///
/// # Returns
/// - `Some(MappedAnswer)` if it matches favorability patterns.
pub fn favorability_answer(label: &str) -> Option<MappedAnswer> {
    let lower = label.to_ascii_lowercase();
    if lower.contains("unfavorable") {
        return Some(answer(
            "unfavorable",
            "Unfavorable",
            net_priority(&lower, "unfavorable"),
        ));
    }
    if lower.contains("favorable") {
        return Some(answer(
            "favorable",
            "Favorable",
            net_priority(&lower, "favorable"),
        ));
    }
    if lower.contains("not heard") {
        return Some(answer("not-heard-of", "Have not heard of them", 3));
    }
    super::unsure_answer(label)
}

/// Determine priority based on whether answer is "net" or exact match.
///
/// # Parameters
/// - `lower`: Lowercase answer label.
/// - `root`: The root word (e.g., "approve").
///
/// # Returns
/// - `3` for net/exact matches, `1` otherwise.
fn net_priority(lower: &str, root: &str) -> u8 {
    if text::is_net_or_exact(lower, root) {
        3
    } else {
        1
    }
}
