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

pub fn get_openapi() -> OpenApiRouter<crate::AppState> {
    OpenApiRouter::with_openapi(AuthDoc::openapi())
        .routes(routes!(google::get_google_login))
        .routes(routes!(google::get_google_callback))
        .routes(routes!(login::post_login))
        .routes(routes!(signup::post_signup))
}

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
