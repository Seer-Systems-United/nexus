mod callback;
mod client;
mod session;

use axum::extract::{Query, State};
use axum::response::Redirect;
use openidconnect::core::CoreAuthenticationFlow;
use openidconnect::{CsrfToken, Nonce, PkceCodeChallenge, Scope};
use serde::Deserialize;

use crate::AppState;
use crate::api::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[utoipa::path(
    get,
    path = "/google/login",
    tag = "Auth",
    responses(
        (status = 302, description = "Redirect to Google OpenID Connect"),
        (status = 503, description = "Google OpenID Connect is not configured", body = crate::api::error::ApiErrorBody),
    )
)]
pub async fn get_google_login(State(state): State<AppState>) -> Result<Redirect, ApiError> {
    let config = state.google_oidc.clone();
    let http_client = client::http_client()?;
    let provider_metadata = client::discover_google_provider(&config, &http_client).await?;
    let client = client::google_client(&config, provider_metadata)?;

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, csrf_state, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    session::store_oidc_state(&state, csrf_state, nonce, pkce_verifier)?;

    Ok(Redirect::temporary(authorize_url.as_str()))
}

#[utoipa::path(
    get,
    path = "/google/callback",
    tag = "Auth",
    params(
        ("code" = Option<String>, Query, description = "Authorization code returned by Google"),
        ("state" = Option<String>, Query, description = "CSRF state returned by Google"),
        ("error" = Option<String>, Query, description = "Google authorization error"),
        ("error_description" = Option<String>, Query, description = "Google authorization error description"),
    ),
    responses(
        (status = 302, description = "Redirect to frontend auth callback"),
        (status = 400, description = "Invalid OpenID Connect callback", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Google OpenID Connect or database is unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
pub async fn get_google_callback(
    State(state): State<AppState>,
    Query(query): Query<GoogleCallbackQuery>,
) -> Result<Redirect, ApiError> {
    if let Some(error) = query.error {
        let description = query.error_description.unwrap_or(error);
        return Err(ApiError::bad_request(description));
    }

    let code = query
        .code
        .ok_or_else(|| ApiError::bad_request("missing google authorization code"))?;
    let csrf_state = query
        .state
        .ok_or_else(|| ApiError::bad_request("missing google authorization state"))?;

    callback::handle_callback(state, code, csrf_state).await
}
