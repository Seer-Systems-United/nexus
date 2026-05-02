//! # Emerson topic matching
//!
//! Maps Emerson data structures to canonical topics.

use crate::sources::DataStructure;
use crate::topics::mappings::{TopicMatch, common};

pub(crate) fn match_structure(structure: &DataStructure) -> Option<TopicMatch> {
    common::generic_match(structure)
}
