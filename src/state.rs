use crate::database::{DbPool, get_connection_pool};
use crate::utils::jwt::JwtConfig;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Option<DbPool>,
    pub jwt: JwtConfig,
}

impl AppState {
    pub fn from_env() -> Self {
        let db_pool = std::env::var("DATABASE_URL")
            .ok()
            .map(|_| get_connection_pool());

        Self {
            db_pool,
            jwt: JwtConfig::from_env(),
        }
    }
}
