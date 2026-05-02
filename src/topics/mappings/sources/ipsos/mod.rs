//! # Ipsos topic matching
//!
//! Maps Ipsos data structures to canonical topics.
//! Handles various Ipsos-specific question patterns.

use crate::sources::DataStructure;
use crate::topics::catalog::{
    ECONOMY_APPROVAL_ID, FOREIGN_POLICY_APPROVAL_ID, GENERIC_BALLOT_ID, IMMIGRATION_APPROVAL_ID,
    IMPORTANT_PROBLEM_ID, INFLATION_APPROVAL_ID, PRESIDENTIAL_APPROVAL_ID, RIGHT_DIRECTION_ID,
    TRUMP_FAVORABILITY_ID,
};
use crate::topics::mappings::{TopicMatch, common, stable_match};
use crate::topics::types::Compatibility;

pub(crate) fn match_structure(structure: &DataStructure) -> Option<TopicMatch> {
    let text = common::structure_text(structure);

    if text.contains("approval5_sum.") {
        return stable_match(PRESIDENTIAL_APPROVAL_ID, Compatibility::ExactWording);
    }

    if text.contains("approval5_1.") || text.contains("approval5_2.") {
        return None;
    }

    if text.contains("cp1.") {
        return stable_match(IMPORTANT_PROBLEM_ID, Compatibility::ExactWording);
    }

    if text.contains("cp2.") {
        return stable_match(RIGHT_DIRECTION_ID, Compatibility::ExactWording);
    }

    if text.contains("generally speaking, would you say the following things") {
        return None;
    }

    if text.contains("tm3287y24.") {
        return stable_match(GENERIC_BALLOT_ID, Compatibility::ExactWording);
    }

    if text.contains("tm1128y17_1.") {
        return stable_match(ECONOMY_APPROVAL_ID, Compatibility::ExactWording);
    }

    if text.contains("tm1128y17_8.") || text.contains("handling inflation") {
        return stable_match(INFLATION_APPROVAL_ID, Compatibility::ExactWording);
    }

    if text.contains("tm1128y17_7.") {
        return stable_match(IMMIGRATION_APPROVAL_ID, Compatibility::ExactWording);
    }

    if text.contains("tm1128y17_25.") {
        return stable_match(FOREIGN_POLICY_APPROVAL_ID, Compatibility::ExactWording);
    }

    if text.contains("tm3154y23_1.") && text.contains("donald trump") {
        return stable_match(TRUMP_FAVORABILITY_ID, Compatibility::ExactWording);
    }

    common::generic_match(structure)
}
