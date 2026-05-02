//! # JWT utilities
//!
//! Handles JWT (JSON Web Token) generation and validation for Nexus
//! authentication. Uses HS256 signing algorithm with configurable
//! audience, issuer, and TTL.
//!
//! ## Environment variables
//!
//! - `JWT_SECRET`: Signing secret (defaults to development secret).
//! - `JWT_AUDIENCE`: Token audience (defaults to `nexus`).
//! - `JWT_ISSUER`: Token issuer (defaults to `nexus-api`).
//! - `JWT_TTL_SECONDS`: Token time-to-live in seconds (defaults to 12 hours).

use serde::{Deserialize, Serialize};

const DEFAULT_JWT_AUDIENCE: &str = "nexus";
const DEFAULT_JWT_ISSUER: &str = "nexus-api";
const DEFAULT_JWT_TTL_SECONDS: u64 = 60 * 60 * 12;

/// JWT generation and validation configuration.
///
/// Stores the signing secret, expected audience/issuer, and token TTL.
///
/// # Fields
///
/// - `secret`: HMAC signing secret for HS256 algorithm.
/// - `audience`: Expected `aud` claim value (who the token is intended for).
/// - `issuer`: Expected `iss` claim value (who issued the token).
/// - `ttl_seconds`: Token validity duration in seconds.
#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub audience: String,
    pub issuer: String,
    pub ttl_seconds: u64,
}

impl JwtConfig {
    /// Reads JWT configuration from environment variables.
    ///
    /// # Environment variables
    ///
    /// - `JWT_SECRET`: Signing secret (defaults to development secret).
    /// - `JWT_AUDIENCE`: Token audience (defaults to `nexus`).
    /// - `JWT_ISSUER`: Token issuer (defaults to `nexus-api`).
    /// - `JWT_TTL_SECONDS`: TTL in seconds (defaults to 43200 = 12 hours).
    ///
    /// # Returns
    ///
    /// A configured `JwtConfig` instance.
    pub fn from_env() -> Self {
        let ttl_seconds = std::env::var("JWT_TTL_SECONDS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(DEFAULT_JWT_TTL_SECONDS);

        Self {
            secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "nexus-development-secret-change-me".to_string()),
            audience: std::env::var("JWT_AUDIENCE")
                .unwrap_or_else(|_| DEFAULT_JWT_AUDIENCE.to_string()),
            issuer: std::env::var("JWT_ISSUER").unwrap_or_else(|_| DEFAULT_JWT_ISSUER.to_string()),
            ttl_seconds,
        }
    }
}

/// JWT claims structure for Nexus authentication tokens.
///
/// Follows standard JWT registered claims plus `sub` for user identification.
///
/// # Fields
///
/// - `aud`: Audience (must match `JwtConfig::audience`).
/// - `exp`: Expiration time (Unix timestamp).
/// - `iat`: Issued-at time (Unix timestamp).
/// - `iss`: Issuer (must match `JwtConfig::issuer`).
/// - `nbf`: Not-before time (Unix timestamp, same as `iat`).
/// - `sub`: Subject (user UUID as string).
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub nbf: usize,
    pub sub: String,
}

/// Issues a new JWT token for the given user ID.
///
/// # Parameters
///
/// - `config`: JWT configuration containing secret, audience, issuer, TTL.
/// - `user_id`: UUID of the authenticated user (stored in `sub` claim).
///
/// # Returns
///
/// Returns `Ok(token_string)` on success.
/// Returns `Err` if token encoding fails.
///
/// # Token claims
///
/// - `sub`: User UUID as string.
/// - `exp`: Current time + `config.ttl_seconds`.
/// - `iat`/`nbf`: Current time.
/// - `aud`/`iss`: From `config`.
pub fn issue_token(
    config: &JwtConfig,
    user_id: uuid::Uuid,
) -> jsonwebtoken::errors::Result<String> {
    let now = jsonwebtoken::get_current_timestamp() as usize;
    let claims = JwtClaims {
        aud: config.audience.clone(),
        exp: now + config.ttl_seconds as usize,
        iat: now,
        iss: config.issuer.clone(),
        nbf: now,
        sub: user_id.to_string(),
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(config.secret.as_bytes()),
    )
}

/// Verifies a JWT token and returns the decoded claims.
///
/// # Parameters
///
/// - `config`: JWT configuration for validation.
/// - `token`: The JWT string to verify.
///
/// # Validation checks
///
/// - Signature validity (HS256 with `config.secret`).
/// - Audience matches `config.audience`.
/// - Issuer matches `config.issuer`.
/// - Token is not expired (`exp` claim).
/// - Token is not used before `nbf` claim.
///
/// # Returns
///
/// Returns `Ok(JwtClaims)` if the token is valid.
/// Returns `Err` if verification fails (invalid signature, expired, etc.).
pub fn verify_token(config: &JwtConfig, token: &str) -> jsonwebtoken::errors::Result<JwtClaims> {
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_audience(&[config.audience.as_str()]);
    validation.set_issuer(&[config.issuer.as_str()]);

    jsonwebtoken::decode::<JwtClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(config.secret.as_bytes()),
        &validation,
    )
    .map(|token_data| token_data.claims)
}
