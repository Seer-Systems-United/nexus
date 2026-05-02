//! # Topics module
//!
//! Handles canonical polling question topic classification, NLP enrichment,
//! demographic mapping, answer aggregation, and headline generation.
//!
//! ## Module structure
//!
//! - `catalog`: Topic catalog definitions and lookups.
//! - `service`: Topic classification service and pooling.
//! - `types`: Core topic data types (observations, demographics, topics).
//! - `answers`: Answer classification and aggregation.
//! - `demographics`: Demographic group labels and mappings.
//! - `enrichment`: NLP-based topic enrichment and CLI tools.
//! - `mappings`: Topic-to-demographic mappings.
//! - `nlp`: Natural language processing for topic classification.

pub mod catalog;
pub mod service;
pub mod types;

pub mod answers;
pub mod demographics;
pub mod enrichment;
pub mod mappings;
pub mod nlp;
