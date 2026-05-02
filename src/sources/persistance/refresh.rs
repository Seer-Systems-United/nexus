use super::cache::{CacheSnapshot, SourceCache};
use crate::sources::{DataCollection, Scope, Source};
use std::time::SystemTime;
use tracing::warn;

type DynError = Box<dyn std::error::Error + Send + Sync>;

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
