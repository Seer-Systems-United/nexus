//! # Health check module
//!
//! Provides a simple health check endpoint for service monitoring and
//! load balancer health checks.

/// Health check handler that returns a static "ok" response.
///
/// # Returns
///
/// A static string `"ok"` indicating the service is running.
///
/// # Usage
///
/// Mapped to the `GET /health` endpoint in the main router.
pub async fn health() -> &'static str {
    "ok"
}
