//! # Google OIDC callback handler
//!
//! Processes Google OAuth callbacks, verifies tokens, and issues JWTs.

mod claims;
mod redirect;

use axum::response::Redirect;
use openidconnect::AuthorizationCode;

use crate::AppState;
use crate::api::auth::auth_response;
use crate::api::db;
use crate::api::error::ApiError;
use crate::database::ops::user;

pub(super) async fn handle_callback(
    state: AppState,
    code: String,
    csrf_state: String,
) -> Result<Redirect, ApiError> {
    let oidc_state = super::session::take_oidc_state(&state, &csrf_state)?;
    let config = state.google_oidc.clone();
    let http_client = super::client::http_client()?;
    let provider_metadata = super::client::discover_google_provider(&config, &http_client).await?;
    let client = super::client::google_client(&config, provider_metadata)?;

    let token_response = client
        .exchange_code(AuthorizationCode::new(code))
        .map_err(|_| ApiError::internal("failed to prepare google token exchange"))?
        .set_pkce_verifier(oidc_state.pkce_verifier)
        .request_async(&http_client)
        .await
        .map_err(|_| ApiError::unauthorized("failed to exchange google authorization code"))?;
    let id_token = token_response
        .extra_fields()
        .id_token()
        .ok_or_else(|| ApiError::unauthorized("google did not return an id token"))?;
    let id_token_verifier = client.id_token_verifier();
    let claims = id_token
        .claims(&id_token_verifier, &oidc_state.nonce)
        .map_err(|_| ApiError::unauthorized("failed to verify google id token"))?;

    claims::verify_access_token_hash(&token_response, id_token, &id_token_verifier, claims)?;
    if claims.email_verified() == Some(false) {
        return Err(ApiError::unauthorized("google email is not verified"));
    }

    let email = claims
        .email()
        .map(|email| email.as_str().to_owned())
        .ok_or_else(|| ApiError::unauthorized("google did not return an email address"))?;
    let name = claims::google_display_name(claims, &email);
    let user = db::run(state.clone(), move |conn| {
        user::get_or_create_user_by_email(conn, &name, &email).map_err(ApiError::database)
    })
    .await?;
    let auth = auth_response(&state.jwt, user)?;
    let redirect_url = redirect::frontend_callback_url(&state.google_oidc, auth);

    Ok(Redirect::temporary(&redirect_url))
}
