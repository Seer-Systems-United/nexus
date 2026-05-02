//! # Source cache storage
//!
//! Manages cache snapshots for polling sources, including
//! reading/writing binary cache files using postcard serialization.

mod files;

use crate::sources::{DataCollection, Scope, Source};
use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
    time::SystemTime,
};
use tracing::{debug, warn};

type DynError = Box<dyn std::error::Error + Send + Sync>;

/// A cached data snapshot with its modification time.
///
/// # Fields
/// - `data`: The cached polling data.
/// - `modified`: When the cache was last modified.
#[derive(Clone)]
pub struct CacheSnapshot {
    pub data: DataCollection,
    pub modified: SystemTime,
}

/// Indicates whether a cache snapshot is fresh or stale.
pub(super) struct SnapshotState {
    pub(super) cache: CacheSnapshot,
    pub(super) fresh: bool,
}

/// Cache directory handle for a specific source and scope.
///
/// # Type Parameters
/// - `S`: The source type implementing `Source`.
pub(super) struct SourceCache<S: Source> {
    dir: PathBuf,
    scope: Scope,
    _phantom: PhantomData<S>,
}

impl<S: Source> SourceCache<S> {
    /// Open or create the cache directory for this source and scope.
    pub(super) fn open(scope: Scope) -> Result<Self, DynError> {
        let dir = files::scoped_cache_dir::<S>(scope);
        std::fs::create_dir_all(&dir)?;
        Ok(Self {
            dir,
            scope,
            _phantom: PhantomData,
        })
    }

    /// Get the cache directory path.
    pub(super) fn dir(&self) -> &Path {
        &self.dir
    }

    /// Find the latest cache snapshot, if any.
    ///
    /// # Parameters
    /// - `now`: Current system time for freshness checking.
    ///
    /// # Returns
    /// - `Ok(Some(SnapshotState))`: Latest snapshot with freshness flag.
    /// - `Ok(None)`: No cache entries found.
    pub(super) fn latest_snapshot(
        &self,
        now: SystemTime,
    ) -> Result<Option<SnapshotState>, DynError> {
        for entry in files::cache_entries(&self.dir)? {
            match files::read_cache(&entry.path) {
                Ok(data) => {
                    let fresh = files::is_fresh(entry.modified, now);
                    self.log_cache_hit(&entry, fresh);
                    return Ok(Some(SnapshotState {
                        cache: CacheSnapshot {
                            data,
                            modified: entry.modified,
                        },
                        fresh,
                    }));
                }
                Err(error) => {
                    warn!(
                        source = S::NAME,
                        scope = %self.scope,
                        path = %entry.path.display(),
                        error = %error,
                        "failed to read source cache snapshot"
                    );
                }
            }
        }

        Ok(None)
    }

    /// Write data to a new cache file.
    pub(super) fn write(&self, now: SystemTime, data: &DataCollection) -> Result<(), DynError> {
        files::write_cache(&self.dir, now, data)
    }

    fn log_cache_hit(&self, entry: &files::CacheEntry, fresh: bool) {
        let message = if fresh {
            "serving source data from fresh cache"
        } else {
            "source cache is stale; refreshing"
        };

        debug!(
            source = S::NAME,
            scope = %self.scope,
            path = %entry.path.display(),
            "{message}"
        );
    }
}
