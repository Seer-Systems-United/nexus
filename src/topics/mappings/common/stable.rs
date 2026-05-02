//! # Stable topic matching
//!
//! Matches questions to stable canonical topics.
//! Checks for approval, direction, ballot, and problem topics.

use super::structure_text;
use crate::sources::DataStructure;
use crate::topics::catalog::{
    ECONOMY_APPROVAL_ID, FOREIGN_POLICY_APPROVAL_ID, GENERIC_BALLOT_ID, IMMIGRATION_APPROVAL_ID,
    IMPORTANT_PROBLEM_ID, INFLATION_APPROVAL_ID, PRESIDENTIAL_APPROVAL_ID, RIGHT_DIRECTION_ID,
    TRUMP_FAVORABILITY_ID,
};
use crate::topics::mappings::{TopicMatch, stable_match};
use crate::topics::types::Compatibility;

pub(crate) fn generic_match(structure: &DataStructure) -> Option<TopicMatch> {
    let text = structure_text(structure);

    if is_presidential_approval(&text) {
        return stable_match(PRESIDENTIAL_APPROVAL_ID, Compatibility::EquivalentWording);
    }
    if text.contains("right direction") && text.contains("wrong track") {
        return stable_match(RIGHT_DIRECTION_ID, Compatibility::ExactWording);
    }
    if is_generic_ballot(&text) {
        return stable_match(GENERIC_BALLOT_ID, Compatibility::EquivalentWording);
    }
    if text.contains("most important problem") || text.contains("main problem facing") {
        return stable_match(IMPORTANT_PROBLEM_ID, Compatibility::EquivalentWording);
    }
    if is_economy_approval(&text) {
        return stable_match(ECONOMY_APPROVAL_ID, Compatibility::EquivalentWording);
    }
    if text.contains("inflation")
        || text.contains("rising prices")
        || text.contains("cost of living")
    {
        return stable_match(INFLATION_APPROVAL_ID, Compatibility::EquivalentWording);
    }
    if text.contains("immigration") && (text.contains("approve") || text.contains("handling")) {
        return stable_match(IMMIGRATION_APPROVAL_ID, Compatibility::EquivalentWording);
    }
    if text.contains("foreign policy") && (text.contains("approve") || text.contains("handling")) {
        return stable_match(FOREIGN_POLICY_APPROVAL_ID, Compatibility::EquivalentWording);
    }
    if text.contains("donald trump") && text.contains("favorable") && text.contains("unfavorable") {
        return stable_match(TRUMP_FAVORABILITY_ID, Compatibility::EquivalentWording);
    }

    None
}

fn is_presidential_approval(text: &str) -> bool {
    (text.contains("approve or disapprove")
        || text.contains("approval")
        || text.contains("job performance"))
        && (text.contains("president")
            || text.contains("donald trump")
            || text.contains("trump approval"))
        && !text.contains("issue?")
        && !text.contains("approval on issues")
        && !text.contains("handling the u.s. economy")
        && !text.contains("handling immigration")
        && !text.contains("handling the following issues")
        && !text.contains("handling the following things")
        && !text.contains("handling the situation in iran")
}

fn is_generic_ballot(text: &str) -> bool {
    (text.contains("u.s. congress")
        || text.contains("congressional")
        || text.contains("generic ballot"))
        && text.contains("democratic")
        && text.contains("republican")
}

fn is_economy_approval(text: &str) -> bool {
    text.contains("handling the u.s. economy")
        || text.contains("handling the economy")
        || text.contains("u.s. economy")
}
