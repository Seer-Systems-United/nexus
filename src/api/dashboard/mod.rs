//! # Dashboard endpoint handler
//!
//! Provides authenticated dashboard access with user info and metrics.
//! Requires Bearer JWT token for authorization.

use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppState;
use crate::api::auth::types::UserResponse;
use crate::api::dashboard::types::{DashboardMetric, DashboardResponse};
use crate::api::db;
use crate::api::error::ApiError;
use crate::database::ops::user;

pub mod types;

#[derive(OpenApi)]
#[openapi(
    paths(get_dashboard),
    components(schemas(
        types::DashboardMetric,
        types::DashboardResponse,
        crate::api::auth::types::UserResponse,
        crate::api::error::ApiErrorBody,
    )),
    tags((name = "Dashboard", description = "Authenticated dashboard access"))
)]
struct DashboardDoc;

/// Get the OpenAPI router for dashboard endpoints.
pub fn get_openapi() -> OpenApiRouter<crate::AppState> {
    OpenApiRouter::with_openapi(DashboardDoc::openapi()).routes(routes!(get_dashboard))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "Dashboard",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Dashboard data", body = DashboardResponse),
        (status = 401, description = "Missing or invalid bearer token", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Database unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
/// Handle GET /dashboard to return user info and metrics.
///
/// # Parameters
/// - `state`: Shared application state with DB pool and JWT config.
/// - `headers`: HTTP headers containing the Bearer token.
///
/// # Returns
/// - `Ok(Json<DashboardResponse>)`: User details and dashboard metrics.
///
/// # Errors
/// - `401 Unauthorized`: Missing or invalid bearer token.
/// - `503 Service Unavailable`: Database operation failed.
pub async fn get_dashboard(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<DashboardResponse>, ApiError> {
    let user_id = authorize_user(&state, &headers)?;
    let user = db::run(state, move |conn| {
        user::get_user_by_id(conn, &user_id).map_err(ApiError::database)
    })
    .await?;

    Ok(Json(DashboardResponse {
        user: UserResponse::from(user),
        metrics: vec![
            DashboardMetric {
                label: "Active federations".to_string(),
                value: "24".to_string(),
                status: "online".to_string(),
            },
            DashboardMetric {
                label: "Node availability".to_string(),
                value: "98.7%".to_string(),
                status: "online".to_string(),
            },
            DashboardMetric {
                label: "Ballots synchronized".to_string(),
                value: "12k".to_string(),
                status: "review".to_string(),
            },
        ],
    }))
}

/// Extract and verify the user ID from the Bearer token in headers.
///
/// # Parameters
/// - `state`: Application state containing JWT secret.
/// - `headers`: HTTP headers to extract the Authorization header from.
///
/// # Returns
/// - `Ok(Uuid)`: The authenticated user's ID.
///
/// # Errors
/// - `401 Unauthorized`: Missing or invalid bearer token.
fn authorize_user(state: &AppState, headers: &HeaderMap) -> Result<uuid::Uuid, ApiError> {
    let token = bearer_token(headers)?;
    let claims = crate::utils::jwt::verify_token(&state.jwt, token)
        .map_err(|_| ApiError::unauthorized("invalid bearer token"))?;

    uuid::Uuid::parse_str(&claims.sub).map_err(|_| ApiError::unauthorized("invalid bearer token"))
}

/// Extract the Bearer token from the Authorization header.
///
/// # Parameters
/// - `headers`: HTTP headers containing the Authorization header.
///
/// # Returns
/// - `Ok(&str)`: The Bearer token string (without "Bearer " prefix).
///
/// # Errors
/// - `401 Unauthorized`: Missing or malformed Authorization header.
fn bearer_token(headers: &HeaderMap) -> Result<&str, ApiError> {
    let header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| ApiError::unauthorized("missing bearer token"))?;

    header
        .strip_prefix("Bearer ")
        .filter(|token| !token.is_empty())
        .ok_or_else(|| ApiError::unauthorized("missing bearer token"))
}
