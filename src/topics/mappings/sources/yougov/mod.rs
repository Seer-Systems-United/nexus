//! # YouGov topic matching
//!
//! Maps YouGov data structures to canonical topics.
//! Uses common matching with YouGov-specific overrides.

use crate::sources::DataStructure;
use crate::topics::catalog::{PRESIDENTIAL_APPROVAL_ID, RIGHT_DIRECTION_ID};
use crate::topics::mappings::{TopicMatch, common, stable_match};
use crate::topics::types::Compatibility;

pub(crate) fn match_structure(structure: &DataStructure) -> Option<TopicMatch> {
    let text = common::structure_text(structure);

    if text.contains("direction of country")
        || (text.contains("right direction") && text.contains("wrong track"))
    {
        return stable_match(RIGHT_DIRECTION_ID, Compatibility::ExactWording);
    }

    if (text.contains("trump approval") || text.contains("donald trump approval"))
        && !text.contains("approval on issues")
        && !text.contains("trump approval -")
        && !text.contains("handling the following things")
    {
        return stable_match(PRESIDENTIAL_APPROVAL_ID, Compatibility::EquivalentWording);
    }

    common::generic_match(structure)
}
