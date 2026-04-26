use serde::Serialize;
use utoipa::ToSchema;

use crate::api::auth::types::UserResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardResponse {
    pub user: UserResponse,
    pub metrics: Vec<DashboardMetric>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardMetric {
    pub label: String,
    pub value: String,
    pub status: String,
}
