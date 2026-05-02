use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::reqwest;
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};

use crate::api::error::ApiError;
use crate::state::GoogleOidcConfig;

pub(super) type GoogleClient = CoreClient<
    openidconnect::EndpointSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointMaybeSet,
    openidconnect::EndpointMaybeSet,
>;

pub(super) fn http_client() -> Result<reqwest::Client, ApiError> {
    reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| ApiError::internal("failed to build google openid http client"))
}

pub(super) async fn discover_google_provider(
    config: &GoogleOidcConfig,
    http_client: &reqwest::Client,
) -> Result<CoreProviderMetadata, ApiError> {
    let issuer_url = IssuerUrl::new(config.issuer_url.clone())
        .map_err(|_| ApiError::internal("invalid google openid issuer url"))?;

    CoreProviderMetadata::discover_async(issuer_url, http_client)
        .await
        .map_err(|_| ApiError::service_unavailable("failed to discover google openid provider"))
}

pub(super) fn google_client(
    config: &GoogleOidcConfig,
    provider_metadata: CoreProviderMetadata,
) -> Result<GoogleClient, ApiError> {
    let (client_id, client_secret) = google_credentials(config)?;
    let redirect_url = RedirectUrl::new(config.redirect_url.clone())
        .map_err(|_| ApiError::internal("invalid google redirect url"))?;

    Ok(
        CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
            .set_redirect_uri(redirect_url),
    )
}

fn google_credentials(config: &GoogleOidcConfig) -> Result<(ClientId, ClientSecret), ApiError> {
    let client_id = configured_secret(&config.client_id, "GOOGLE_CLIENT_ID")?;
    let client_secret = configured_secret(&config.client_secret, "GOOGLE_CLIENT_SECRET")?;

    Ok((ClientId::new(client_id), ClientSecret::new(client_secret)))
}

fn configured_secret(value: &Option<String>, name: &'static str) -> Result<String, ApiError> {
    value
        .clone()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            ApiError::service_unavailable(format!("{name} is required for google login"))
        })
}
