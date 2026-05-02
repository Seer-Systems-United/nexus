//! # Answer support models
//!
//! Data structures for mapped and aggregated answers.

#[derive(Debug, Clone)]
/// A mapped answer with canonical ID, label, and priority.
pub struct MappedAnswer {
    pub id: String,
    pub label: String,
    pub priority: u8,
}

#[derive(Debug, Clone)]
/// An aggregated answer with accumulated value and priority.
pub struct AggregatedAnswer {
    pub label: String,
    pub value: f32,
    pub priority: u8,
}
