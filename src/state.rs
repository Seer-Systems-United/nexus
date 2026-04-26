use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::database::{DbPool, get_connection_pool};
use crate::utils::jwt::JwtConfig;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Option<DbPool>,
    pub google_oidc: GoogleOidcConfig,
    pub google_oidc_sessions: Arc<Mutex<HashMap<String, OidcLoginState>>>,
    pub jwt: JwtConfig,
}

impl AppState {
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

#[derive(Clone)]
pub struct GoogleOidcConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub issuer_url: String,
    pub redirect_url: String,
    pub success_redirect_path: String,
}

impl GoogleOidcConfig {
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

pub struct OidcLoginState {
    pub nonce: openidconnect::Nonce,
    pub pkce_verifier: openidconnect::PkceCodeVerifier,
    pub created_at: Instant,
}
