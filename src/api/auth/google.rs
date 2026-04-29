use std::time::{Duration, Instant};

use axum::extract::{Query, State};
use axum::response::Redirect;
use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata};
use openidconnect::reqwest;
use openidconnect::{
    AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, PkceCodeChallenge, RedirectUrl, Scope,
};
use serde::Deserialize;

use crate::AppState;
use crate::api::auth::auth_response;
use crate::api::auth::types::{AuthResponse, UserResponse};
use crate::api::db;
use crate::api::error::ApiError;
use crate::database::ops::user;
use crate::state::{GoogleOidcConfig, OidcLoginState};

const GOOGLE_OIDC_STATE_TTL: Duration = Duration::from_secs(10 * 60);

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
    let http_client = http_client()?;
    let provider_metadata = discover_google_provider(&config, &http_client).await?;
    let client = google_client(&config, provider_metadata)?;

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

    store_oidc_state(&state, csrf_state, nonce, pkce_verifier)?;

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
    let oidc_state = take_oidc_state(&state, &csrf_state)?;

    let config = state.google_oidc.clone();
    let http_client = http_client()?;
    let provider_metadata = discover_google_provider(&config, &http_client).await?;
    let client = google_client(&config, provider_metadata)?;

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

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            id_token
                .signing_alg()
                .map_err(|_| ApiError::unauthorized("invalid google id token signing algorithm"))?,
            id_token
                .signing_key(&id_token_verifier)
                .map_err(|_| ApiError::unauthorized("invalid google id token signing key"))?,
        )
        .map_err(|_| ApiError::unauthorized("invalid google access token hash"))?;

        if actual_access_token_hash != *expected_access_token_hash {
            return Err(ApiError::unauthorized("invalid google access token hash"));
        }
    }

    if claims.email_verified() == Some(false) {
        return Err(ApiError::unauthorized("google email is not verified"));
    }

    let email = claims
        .email()
        .map(|email| email.as_str().to_owned())
        .ok_or_else(|| ApiError::unauthorized("google did not return an email address"))?;
    let name = google_display_name(claims, &email);
    let user = db::run(state.clone(), move |conn| {
        user::get_or_create_user_by_email(conn, &name, &email).map_err(ApiError::database)
    })
    .await?;
    let auth = auth_response(&state.jwt, user)?;
    let redirect_url = frontend_callback_url(&state.google_oidc, auth);

    Ok(Redirect::temporary(&redirect_url))
}

fn google_credentials(config: &GoogleOidcConfig) -> Result<(ClientId, ClientSecret), ApiError> {
    let client_id = config
        .client_id
        .clone()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            ApiError::service_unavailable("GOOGLE_CLIENT_ID is required for google login")
        })?;
    let client_secret = config
        .client_secret
        .clone()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            ApiError::service_unavailable("GOOGLE_CLIENT_SECRET is required for google login")
        })?;

    Ok((ClientId::new(client_id), ClientSecret::new(client_secret)))
}

fn http_client() -> Result<reqwest::Client, ApiError> {
    reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| ApiError::internal("failed to build google openid http client"))
}

async fn discover_google_provider(
    config: &GoogleOidcConfig,
    http_client: &reqwest::Client,
) -> Result<CoreProviderMetadata, ApiError> {
    let issuer_url = IssuerUrl::new(config.issuer_url.clone())
        .map_err(|_| ApiError::internal("invalid google openid issuer url"))?;

    CoreProviderMetadata::discover_async(issuer_url, http_client)
        .await
        .map_err(|_| ApiError::service_unavailable("failed to discover google openid provider"))
}

fn google_client(
    config: &GoogleOidcConfig,
    provider_metadata: CoreProviderMetadata,
) -> Result<
    CoreClient<
        openidconnect::EndpointSet,
        openidconnect::EndpointNotSet,
        openidconnect::EndpointNotSet,
        openidconnect::EndpointNotSet,
        openidconnect::EndpointMaybeSet,
        openidconnect::EndpointMaybeSet,
    >,
    ApiError,
> {
    let (client_id, client_secret) = google_credentials(config)?;
    let redirect_url = RedirectUrl::new(config.redirect_url.clone())
        .map_err(|_| ApiError::internal("invalid google redirect url"))?;

    Ok(
        CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
            .set_redirect_uri(redirect_url),
    )
}

fn store_oidc_state(
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

fn take_oidc_state(state: &AppState, csrf_state: &str) -> Result<OidcLoginState, ApiError> {
    let mut sessions = state
        .google_oidc_sessions
        .lock()
        .map_err(|_| ApiError::internal("google login state is unavailable"))?;

    sessions.retain(|_, session| session.created_at.elapsed() < GOOGLE_OIDC_STATE_TTL);
    sessions
        .remove(csrf_state)
        .ok_or_else(|| ApiError::bad_request("invalid or expired google authorization state"))
}

fn google_display_name(
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

fn frontend_callback_url(config: &GoogleOidcConfig, auth: AuthResponse) -> String {
    let user: UserResponse = auth.user;
    let mut fragment = url::form_urlencoded::Serializer::new(String::new());
    fragment
        .append_pair("token", &auth.token)
        .append_pair("token_type", &auth.token_type)
        .append_pair("expires_in", &auth.expires_in.to_string())
        .append_pair("user_id", &user.id)
        .append_pair("user_name", &user.name)
        .append_pair("user_created_at", &user.created_at);

    if let Some(email) = user.email {
        fragment.append_pair("user_email", &email);
    }

    if let Some(account_number) = user.account_number {
        fragment.append_pair("user_account_number", &account_number);
    }

    let fragment = fragment.finish();

    format!("{}#{}", config.success_redirect_path, fragment)
}
