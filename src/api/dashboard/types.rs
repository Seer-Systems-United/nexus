//! # Dashboard API types
//!
//! Defines response structures for the authenticated dashboard endpoint
//! with OpenAPI schema support.

use serde::Serialize;
use utoipa::ToSchema;

use crate::api::auth::types::UserResponse;

/// Dashboard response containing user info and metrics.
///
/// # Fields
/// - `user`: Authenticated user's public details.
/// - `metrics`: List of dashboard metrics to display.
#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardResponse {
    pub user: UserResponse,
    pub metrics: Vec<DashboardMetric>,
}

/// A single metric displayed on the dashboard.
///
/// # Fields
/// - `label`: Human-readable label for the metric.
/// - `value`: Current value of the metric.
/// - `status`: Status indicator (e.g., "online", "review").
#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardMetric {
    pub label: String,
    pub value: String,
    pub status: String,
}
