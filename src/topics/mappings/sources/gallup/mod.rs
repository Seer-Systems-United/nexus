use crate::sources::DataStructure;
use crate::topics::catalog::PRESIDENTIAL_APPROVAL_ID;
use crate::topics::mappings::{TopicMatch, common, stable_match};
use crate::topics::types::Compatibility;

pub(crate) fn match_structure(structure: &DataStructure) -> Option<TopicMatch> {
    let text = common::structure_text(structure);

    if text.contains("president") && text.contains("approval") {
        return stable_match(PRESIDENTIAL_APPROVAL_ID, Compatibility::TrendComparable);
    }

    common::generic_match(structure)
}
