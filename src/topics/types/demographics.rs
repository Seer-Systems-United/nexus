//! # Demographics types
//!
//! Data types for demographic groups and values in polling data.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DemographicGroup {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DemographicValue {
    pub id: String,
    pub label: String,
    pub group: Option<DemographicGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnswerResult {
    pub id: String,
    pub label: String,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DemographicResult {
    pub demographic: DemographicValue,
    pub answers: Vec<AnswerResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PooledAnswerResult {
    pub id: String,
    pub label: String,
    pub value: f32,
    pub observation_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PooledDemographicResult {
    pub demographic: DemographicValue,
    pub answers: Vec<PooledAnswerResult>,
}
