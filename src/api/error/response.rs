//! # API error response integration
//!
//! Implements `IntoResponse` for `ApiError` to convert errors into
//! proper HTTP responses with JSON bodies and appropriate status codes.

use super::{ApiError, ApiErrorBody};
use axum::Json;
use axum::response::{IntoResponse, Response};

/// Convert `ApiError` into an Axum HTTP response.
///
/// Logs server errors at ERROR level and client errors at WARN level.
/// Returns a JSON body with error code and message.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if self.status.is_server_error() {
            tracing::error!(
                status = %self.status,
                code = self.code,
                message = %self.message,
                "api request failed"
            );
        } else if self.status.is_client_error() {
            tracing::warn!(
                status = %self.status,
                code = self.code,
                message = %self.message,
                "api request rejected"
            );
        }

        let body = ApiErrorBody {
            error: self.code.to_string(),
            message: self.message,
        };

        (self.status, Json(body)).into_response()
    }
}
