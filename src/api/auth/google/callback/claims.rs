//! # Google OIDC claims verifier
//!
//! Verifies Google OAuth access token hashes and ID token claims.

use crate::api::error::ApiError;
use openidconnect::{AccessTokenHash, OAuth2TokenResponse};

pub(super) fn verify_access_token_hash(
    token_response: &openidconnect::core::CoreTokenResponse,
    id_token: &openidconnect::core::CoreIdToken,
    verifier: &openidconnect::core::CoreIdTokenVerifier,
    claims: &openidconnect::core::CoreIdTokenClaims,
) -> Result<(), ApiError> {
    let Some(expected_access_token_hash) = claims.access_token_hash() else {
        return Ok(());
    };

    let actual_access_token_hash = AccessTokenHash::from_token(
        token_response.access_token(),
        id_token
            .signing_alg()
            .map_err(|_| ApiError::unauthorized("invalid google id token signing algorithm"))?,
        id_token
            .signing_key(verifier)
            .map_err(|_| ApiError::unauthorized("invalid google id token signing key"))?,
    )
    .map_err(|_| ApiError::unauthorized("invalid google access token hash"))?;

    if actual_access_token_hash != *expected_access_token_hash {
        return Err(ApiError::unauthorized("invalid google access token hash"));
    }

    Ok(())
}

pub(super) fn google_display_name(
    claims: &openidconnect::core::CoreIdTokenClaims,
    fallback_email: &str,
) -> String {
    claims
        .name()
        .and_then(|name| name.get(None))
        .map(|name| name.as_str().to_owned())
        .or_else(|| {
            claims
                .preferred_username()
                .map(|username| username.as_str().to_owned())
        })
        .unwrap_or_else(|| fallback_email.to_owned())
}
