use serde::{Deserialize, Serialize};

const DEFAULT_JWT_AUDIENCE: &str = "nexus";
const DEFAULT_JWT_ISSUER: &str = "nexus-api";
const DEFAULT_JWT_TTL_SECONDS: u64 = 60 * 60 * 12;

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub audience: String,
    pub issuer: String,
    pub ttl_seconds: u64,
}

impl JwtConfig {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub nbf: usize,
    pub sub: String,
}

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
