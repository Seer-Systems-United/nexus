//! # Database helper
//!
//! Provides a `run` helper to execute database operations with error handling.

use crate::AppState;
use crate::api::error::ApiError;
use crate::database::DbConnection;

pub async fn run<T, F>(state: AppState, operation: F) -> Result<T, ApiError>
where
    T: Send + 'static,
    F: FnOnce(&mut DbConnection) -> Result<T, ApiError> + Send + 'static,
{
    let pool = state
        .db_pool
        .ok_or_else(|| ApiError::service_unavailable("database is not configured"))?;

    tokio::task::spawn_blocking(move || {
        let mut conn = pool
            .get()
            .map_err(|_| ApiError::service_unavailable("database connection unavailable"))?;

        operation(&mut conn)
    })
    .await
    .map_err(|_| ApiError::internal("database task failed"))?
}
