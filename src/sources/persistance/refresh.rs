//! # Cache refresh logic
//!
//! Handles fallback to stale cache when fresh data fetch fails.
//! Ensures service availability even when source downloads fail.

use super::cache::{CacheSnapshot, SourceCache};
use crate::sources::{DataCollection, Scope, Source};
use std::time::SystemTime;
use tracing::warn;

type DynError = Box<dyn std::error::Error + Send + Sync>;

/// Return stale cache if available, otherwise propagate the error.
///
/// # Parameters
/// - `cache`: The source cache handle.
/// - `now`: Current system time.
/// - `scope`: The query scope.
/// - `stale_cache`: Optional stale cache data.
/// - `error`: The error that occurred during fresh data fetch.
///
/// # Returns
/// - `Ok(DataCollection)`: Stale cache data if available.
///
/// # Errors
/// - Returns the original error if no stale cache is available.
pub(super) fn stale_or_error<S: Source>(
    cache: &SourceCache<S>,
    now: SystemTime,
    scope: Scope,
    stale_cache: Option<CacheSnapshot>,
    error: DynError,
) -> Result<DataCollection, DynError> {
    let Some(cached) = stale_cache else {
        warn!(
            source = S::NAME,
            scope = %scope,
            error = %error,
            "source refresh failed without usable cache"
        );
        return Err(error);
    };

    warn!(
        source = S::NAME,
        scope = %scope,
        error = %error,
        "source refresh failed; serving stale cache"
    );
    // Refresh the stale cache timestamp so we don't keep retrying
    if let Err(write_error) = cache.write(now, &cached.data) {
        warn!(
            source = S::NAME,
            scope = %scope,
            error = %write_error,
            "failed to refresh stale cache timestamp after source refresh error"
        );
    }

    Ok(cached.data)
}
