//! # Application state module
//!
//! Defines the shared `AppState` struct passed to all Axum handlers,
//! containing database pools, OIDC configuration, JWT settings, and
//! active OIDC login sessions.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::database::{DbPool, get_connection_pool};
use crate::utils::jwt::JwtConfig;

/// Shared application state injected into Axum handlers.
///
/// Contains all shared resources needed by the API and background services:
///
/// - `db_pool`: Optional PostgreSQL connection pool (None if DATABASE_URL not set).
/// - `google_oidc`: Google OpenID Connect configuration.
/// - `google_oidc_sessions`: Active OIDC login sessions (thread-safe via Mutex).
/// - `jwt`: JWT token generation/validation configuration.
///
/// # Clone behavior
///
/// Derived `Clone` implementation (all fields must be `Clone`).
#[derive(Clone)]
pub struct AppState {
    pub db_pool: Option<DbPool>,
    pub google_oidc: GoogleOidcConfig,
    pub google_oidc_sessions: Arc<Mutex<HashMap<String, OidcLoginState>>>,
    pub jwt: JwtConfig,
}

impl AppState {
    /// Initializes `AppState` from environment variables.
    ///
    /// # Environment variables
    ///
    /// - `DATABASE_URL`: If set, initializes the PostgreSQL connection pool.
    /// - `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`: Google OIDC credentials.
    /// - `GOOGLE_REDIRECT_URL`: OIDC callback URL (falls back to default).
    /// - `JWT_*` variables: Configured via `JwtConfig::from_env()`.
    ///
    /// # Behavior
    ///
    /// - Loads `.env` file if present (via `dotenv::dotenv()`).
    /// - OIDC sessions are initialized as an empty thread-safe HashMap.
    ///
    /// # Returns
    ///
    /// A fully configured `AppState` instance.
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let db_pool = std::env::var("DATABASE_URL")
            .ok()
            .map(|_| get_connection_pool());

        Self {
            db_pool,
            google_oidc: GoogleOidcConfig::from_env(),
            google_oidc_sessions: Arc::new(Mutex::new(HashMap::new())),
            jwt: JwtConfig::from_env(),
        }
    }
}

/// Google OpenID Connect configuration.
///
/// Stores client credentials, issuer URL, redirect URL, and post-login
/// redirect path for Google OAuth flows.
///
/// # Fields
///
/// - `client_id`: Optional Google OAuth client ID (None if not configured).
/// - `client_secret`: Optional Google OAuth client secret (None if not configured).
/// - `issuer_url`: OpenID Connect issuer URL (defaults to `https://accounts.google.com`).
/// - `redirect_url`: OAuth callback URL (defaults to local dev URL).
/// - `success_redirect_path`: Frontend path to redirect to after successful login.
#[derive(Clone)]
pub struct GoogleOidcConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub issuer_url: String,
    pub redirect_url: String,
    pub success_redirect_path: String,
}

impl GoogleOidcConfig {
    /// Reads Google OIDC configuration from environment variables.
    ///
    /// # Environment variables
    ///
    /// - `GOOGLE_CLIENT_ID`: Optional OAuth client ID.
    /// - `GOOGLE_CLIENT_SECRET`: Optional OAuth client secret.
    /// - `GOOGLE_OIDC_ISSUER`: OpenID issuer (defaults to `https://accounts.google.com`).
    /// - `GOOGLE_REDIRECT_URL` or `GOOGLE_REDIRECT_URI`: Callback URL (defaults to local dev).
    /// - `GOOGLE_LOGIN_SUCCESS_URL`: Post-login redirect path (defaults to `/auth/callback`).
    ///
    /// # Returns
    ///
    /// A configured `GoogleOidcConfig` instance.
    fn from_env() -> Self {
        Self {
            client_id: std::env::var("GOOGLE_CLIENT_ID").ok(),
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok(),
            issuer_url: std::env::var("GOOGLE_OIDC_ISSUER")
                .unwrap_or_else(|_| "https://accounts.google.com".to_string()),
            redirect_url: std::env::var("GOOGLE_REDIRECT_URL")
                .or_else(|_| std::env::var("GOOGLE_REDIRECT_URI"))
                .unwrap_or_else(|_| {
                    "http://127.0.0.1:8080/api/v1/auth/google/callback".to_string()
                }),
            success_redirect_path: std::env::var("GOOGLE_LOGIN_SUCCESS_URL")
                .unwrap_or_else(|_| "/auth/callback".to_string()),
        }
    }
}

/// Temporary state for an in-progress OIDC login session.
///
/// Stored in `AppState::google_oidc_sessions` until the OAuth callback
/// is processed.
///
/// # Fields
///
/// - `nonce`: OpenID Connect nonce for replay attack prevention.
/// - `pkce_verifier`: PKCE code verifier for the OAuth flow.
/// - `created_at`: Instant when the login session was created (for expiration checks).
pub struct OidcLoginState {
    pub nonce: openidconnect::Nonce,
    pub pkce_verifier: openidconnect::PkceCodeVerifier,
    pub created_at: Instant,
}
