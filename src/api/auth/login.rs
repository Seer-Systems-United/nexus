//! # Login endpoint handler
//!
//! Handles user login requests with account number/password, verifies credentials,
//! and returns JWT authentication responses.

use axum::Json;
use axum::extract::State;

use crate::AppState;
use crate::api::auth::auth_response;
use crate::api::auth::types::{AuthResponse, LoginRequest};
use crate::api::db;
use crate::api::error::ApiError;
use crate::database::ops::password::PasswordError;
use crate::database::ops::{password, user};

#[utoipa::path(
    post,
    path = "/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Database unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
/// Handle POST /login requests to authenticate existing users.
///
/// # Parameters
/// - `state`: Shared application state containing JWT config and DB pool.
/// - `request`: JSON payload with account number and password.
///
/// # Returns
/// - `Ok(Json<AuthResponse>)`: Authenticated user details and JWT token.
///
/// # Errors
/// - `401 Unauthorized`: Invalid credentials (account number not found, invalid password).
/// - `503 Service Unavailable`: Database connection or operation failure.
pub async fn post_login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let account_number = user::normalize_account_number(&request.account_number);

    if !user::is_valid_account_number(&account_number) {
        return Err(ApiError::unauthorized("invalid credentials"));
    }

    let supplied_password = request.password;
    let user = db::run(state.clone(), move |conn| {
        let user =
            user::get_user_by_account_number(conn, &account_number).map_err(
                |error| match error {
                    diesel::result::Error::NotFound => {
                        ApiError::unauthorized("invalid credentials")
                    }
                    error => ApiError::database(error),
                },
            )?;

        let valid_password = password::verify_login_password(conn, user.id, &supplied_password)
            .map_err(|error| match error {
                PasswordError::Database(diesel::result::Error::NotFound) => {
                    ApiError::unauthorized("invalid credentials")
                }
                error => ApiError::password(error),
            })?;

        if !valid_password {
            return Err(ApiError::unauthorized("invalid credentials"));
        }

        Ok(user)
    })
    .await?;

    Ok(Json(auth_response(&state.jwt, user)?))
}
