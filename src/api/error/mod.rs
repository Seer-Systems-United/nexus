//! # API error handling module
//!
//! Defines the `ApiError` type for consistent HTTP error responses
//! across all API endpoints, with integration for Diesel and password errors.

use axum::http::StatusCode;
mod body;
mod response;

use crate::database::ops::password::PasswordError;
pub use body::ApiErrorBody;

/// Structured API error with HTTP status, error code, and message.
///
/// # Fields
/// - `status`: HTTP status code for the response.
/// - `code`: Machine-readable error code string.
/// - `message`: Human-readable error message.
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    /// Create a 400 Bad Request error.
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "bad_request", message)
    }

    /// Create a 401 Unauthorized error.
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "unauthorized", message)
    }

    /// Create a 404 Not Found error.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", message)
    }

    /// Create a 409 Conflict error.
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, "conflict", message)
    }

    /// Create a 503 Service Unavailable error.
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "service_unavailable",
            message,
        )
    }

    /// Create a 500 Internal Server Error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", message)
    }

    /// Convert a Diesel error into an appropriate API error.
    pub fn database(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => Self::unauthorized("resource not found"),
            _ => Self::internal("database operation failed"),
        }
    }

    /// Convert a password error into an appropriate API error.
    pub fn password(error: PasswordError) -> Self {
        match error {
            PasswordError::Database(error) => Self::database(error),
            _ => Self::internal("password operation failed"),
        }
    }

    /// Create a new `ApiError` with the given status, code, and message.
    fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }
}

/// Convert Diesel errors into `ApiError` automatically.
impl From<diesel::result::Error> for ApiError {
    fn from(error: diesel::result::Error) -> Self {
        Self::database(error)
    }
}
