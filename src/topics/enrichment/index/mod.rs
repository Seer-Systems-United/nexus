//! # Enrichment index module
//!
//! Loads, stores, and applies the question enrichment index.
//! Maps question fingerprints to canonical topics.

mod apply;
mod input;
mod storage;

pub use apply::{applicable_topic_id, apply_index_to_observations};
pub(super) use input::classification_inputs;
pub(super) use storage::{index_path_from_env, load_index_from_path, save_index_to_path};
