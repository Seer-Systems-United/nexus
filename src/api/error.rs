use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

use crate::database::ops::password::PasswordError;

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorBody {
    pub error: String,
    pub message: String,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "bad_request", message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "unauthorized", message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, "conflict", message)
    }

    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "service_unavailable",
            message,
        )
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", message)
    }

    pub fn database(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => Self::unauthorized("resource not found"),
            _ => Self::internal("database operation failed"),
        }
    }

    pub fn password(error: PasswordError) -> Self {
        match error {
            PasswordError::Database(error) => Self::database(error),
            _ => Self::internal("password operation failed"),
        }
    }

    fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(error: diesel::result::Error) -> Self {
        Self::database(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ApiErrorBody {
            error: self.code.to_string(),
            message: self.message,
        };

        (self.status, Json(body)).into_response()
    }
}
