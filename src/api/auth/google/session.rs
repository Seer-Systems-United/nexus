use std::time::{Duration, Instant};

use openidconnect::{CsrfToken, Nonce};

use crate::AppState;
use crate::api::error::ApiError;
use crate::state::OidcLoginState;

const GOOGLE_OIDC_STATE_TTL: Duration = Duration::from_secs(10 * 60);

pub(super) fn store_oidc_state(
    state: &AppState,
    csrf_state: CsrfToken,
    nonce: Nonce,
    pkce_verifier: openidconnect::PkceCodeVerifier,
) -> Result<(), ApiError> {
    let mut sessions = state
        .google_oidc_sessions
        .lock()
        .map_err(|_| ApiError::internal("google login state is unavailable"))?;

    sessions.retain(|_, session| session.created_at.elapsed() < GOOGLE_OIDC_STATE_TTL);
    sessions.insert(
        csrf_state.secret().to_owned(),
        OidcLoginState {
            nonce,
            pkce_verifier,
            created_at: Instant::now(),
        },
    );

    Ok(())
}

pub(super) fn take_oidc_state(
    state: &AppState,
    csrf_state: &str,
) -> Result<OidcLoginState, ApiError> {
    let mut sessions = state
        .google_oidc_sessions
        .lock()
        .map_err(|_| ApiError::internal("google login state is unavailable"))?;

    sessions.retain(|_, session| session.created_at.elapsed() < GOOGLE_OIDC_STATE_TTL);
    sessions
        .remove(csrf_state)
        .ok_or_else(|| ApiError::bad_request("invalid or expired google authorization state"))
}
