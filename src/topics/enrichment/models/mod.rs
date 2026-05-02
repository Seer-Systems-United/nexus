//! # Enrichment models module
//!
//! Re-exports classification and record model types.

mod classification;
mod records;

pub use classification::{ClassificationInput, ClassificationOutput};
pub use records::{QuestionEnrichment, QuestionIndex};
