use super::{ApiError, ApiErrorBody};
use axum::Json;
use axum::response::{IntoResponse, Response};

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
