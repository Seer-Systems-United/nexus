//! # API module
//!
//! This module implements the Axum-based HTTP API for the Nexus system.
//! It provides OpenAPI 3.0 documented endpoints for authentication, dashboard
//! access, polling source ingestion, and canonical topic management.
//!
//! ## Module structure
//!
//! - `auth`: Google OpenID Connect and password-based authentication.
//! - `dashboard`: Authenticated user dashboard endpoints.
//! - `error`: API error types and response formatting.
//! - `sources`: Polling source ingestion endpoints (Emerson, Gallup, YouGov).
//! - `topics`: Canonical polling topic management endpoints.
//!
//! ## Authentication
//!
//! All protected endpoints require a Bearer JWT token, configured via the
//! `bearer_auth` security scheme in OpenAPI.
//!
//! ## OpenAPI docs
//!
//! Available at `/docs` via the Scalar UI when the server is running.

use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_axum::router::OpenApiRouter;

pub mod auth;
pub mod dashboard;
mod db;
pub mod error;
pub mod sources;
pub mod topics;

/// OpenAPI documentation struct for the Nexus v1 API.
///
/// This struct is used by `utoipa` to generate the OpenAPI 3.0 specification.
/// It defines the API server URL, security schemes, and endpoint tags.
///
/// # OpenAPI configuration
///
/// - Server URL: `/api`
/// - Security: Bearer JWT (added via `SecurityAddon`)
/// - Tags: Auth, Dashboard, Sources, Topics
#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    servers((url = "/api")),
    tags(
        (name = "Auth", description = "Authentication and account creation"),
        (name = "Dashboard", description = "Authenticated dashboard access"),
        (name = "Sources", description = "Polling source ingestion"),
        (name = "Topics", description = "Canonical polling question topics"),
    )
)]
struct NexusApiDoc;

/// Builds the root OpenAPI router for the Nexus API.
///
/// # Returns
///
/// An `OpenApiRouter` with the full API spec and nested sub-routers for
/// auth, dashboard, sources, and topics endpoints.
///
/// # Sub-routers
///
/// - `/v1/auth`: Authentication endpoints (Google OpenID + password).
/// - `/v1/dashboard`: Protected dashboard endpoints.
/// - `/v1/sources`: Polling source ingestion endpoints.
/// - `/v1/topics`: Canonical topic management endpoints.
pub fn get_openapi() -> OpenApiRouter<crate::AppState> {
    OpenApiRouter::with_openapi(NexusApiDoc::openapi())
        .nest("/v1/auth", auth::get_openapi())
        .nest("/v1/dashboard", dashboard::get_openapi())
        .nest("/v1/sources", sources::get_openapi())
        .nest("/v1/topics", topics::get_openapi())
}

/// OpenAPI security scheme modifier that adds Bearer JWT authentication.
///
/// This struct implements `utoipa::Modify` to inject the `bearer_auth`
/// security scheme into the generated OpenAPI specification.
struct SecurityAddon;

/// Adds Bearer JWT authentication to the OpenAPI security schemes.
///
/// # Security scheme details
///
/// - Scheme: HTTP Bearer
/// - Bearer format: JWT
/// - Scheme name in spec: `bearer_auth`
impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .get_or_insert_with(utoipa::openapi::Components::new)
            .add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
    }
}
