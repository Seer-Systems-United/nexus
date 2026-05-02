//! # Authentication module
//!
//! Handles user authentication via Google OpenID Connect and password-based
//! signup/login. Issues JWT tokens for authenticated sessions.
//!
//! ## Module structure
//!
//! - `google`: Google OIDC login flow (login, callback, session management).
//! - `login`: Password-based login endpoint.
//! - `signup`: Password-based signup endpoint.
//! - `types`: Auth request/response types (login, signup, JWT response).
//!
//! ## Endpoints
//!
//! - `GET /v1/auth/google/login`: Initiate Google OIDC flow.
//! - `GET /v1/auth/google/callback`: Google OIDC callback handler.
//! - `POST /v1/auth/login`: Password-based login.
//! - `POST /v1/auth/signup`: Password-based signup.

use crate::api::error::ApiError;
use crate::database::schema::user::User;
use crate::utils::jwt::JwtConfig;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

pub mod google;
pub mod login;
pub mod signup;
pub mod types;

use types::AuthResponse;

/// OpenAPI documentation struct for the Auth API section.
///
/// Defines the paths, components, and tags for authentication endpoints.
#[derive(OpenApi)]
#[openapi(
    paths(
        google::get_google_callback,
        google::get_google_login,
        login::post_login,
        signup::post_signup
    ),
    components(schemas(
        types::AuthResponse,
        types::LoginRequest,
        types::SignupRequest,
        types::UserResponse,
        crate::api::error::ApiErrorBody,
    )),
    tags((name = "Auth", description = "Authentication and account creation"))
)]
struct AuthDoc;

/// Builds the Auth API sub-router with OpenAPI documentation.
///
/// # Returns
///
/// An `OpenApiRouter` with all auth endpoints and their OpenAPI specs.
///
/// # Included endpoints
///
/// - `GET /google/login`: Initiate Google OIDC flow.
/// - `GET /google/callback`: Google OIDC callback.
/// - `POST /login`: Password-based login.
/// - `POST /signup`: Password-based signup.
pub fn get_openapi() -> OpenApiRouter<crate::AppState> {
    OpenApiRouter::with_openapi(AuthDoc::openapi())
        .routes(routes!(google::get_google_login))
        .routes(routes!(google::get_google_callback))
        .routes(routes!(login::post_login))
        .routes(routes!(signup::post_signup))
}

/// Generates a standardized authentication response with JWT token.
///
/// # Parameters
///
/// - `jwt`: JWT configuration for token issuance.
/// - `user`: The authenticated user record.
///
/// # Returns
///
/// Returns `Ok(AuthResponse)` with token, type, expiry, and user info.
/// Returns `Err(ApiError)` if token issuance fails.
fn auth_response(jwt: &JwtConfig, user: User) -> Result<AuthResponse, ApiError> {
    let token = crate::utils::jwt::issue_token(jwt, user.id)
        .map_err(|_| ApiError::internal("failed to issue token"))?;

    Ok(AuthResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: jwt.ttl_seconds,
        user: user.into(),
    })
}
