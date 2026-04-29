use crate::sources::{DataCollection, Scope, Source};
use std::{
    future::Future,
    marker::PhantomData,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tracing::{debug, warn};

const REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 60); // 1 hour

type DynError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone)]
pub struct CacheSnapshot {
    pub data: DataCollection,
    pub modified: SystemTime,
}

#[derive(Clone)]
struct CacheEntry {
    path: PathBuf,
    modified: SystemTime,
}

pub struct StorageWrapper<S: Source> {
    _phantom: PhantomData<S>,
}

impl<S: Source> StorageWrapper<S> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub async fn get_data<F, Fut>(&self, scope: Scope, fetch: F) -> Result<DataCollection, DynError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<DataCollection, DynError>>,
    {
        self.get_data_with_cache(scope, |_| fetch()).await
    }

    pub async fn get_data_with_cache<F, Fut>(
        &self,
        scope: Scope,
        fetch: F,
    ) -> Result<DataCollection, DynError>
    where
        F: FnOnce(Option<CacheSnapshot>) -> Fut,
        Fut: Future<Output = Result<DataCollection, DynError>>,
    {
        // Each source gets its own cache directory under `data/<source-name>`.
        let cache_dir = Self::ensure_cache_dir(scope)?;
        debug!(
            source = S::NAME,
            scope = %scope,
            cache_dir = %cache_dir.display(),
            "opened source cache directory"
        );
        let now = SystemTime::now();
        let mut stale_cache = None;

        // Try to reuse the most recent readable cached file before hitting the source again.
        for entry in Self::cache_entries(&cache_dir)? {
            match Self::read_cache(&entry.path) {
                Ok(cached) => {
                    let snapshot = CacheSnapshot {
                        data: cached,
                        modified: entry.modified,
                    };

                    // Fresh cache wins immediately.
                    if Self::is_fresh(entry.modified, now) {
                        debug!(
                            source = S::NAME,
                            scope = %scope,
                            path = %entry.path.display(),
                            "serving source data from fresh cache"
                        );
                        return Ok(snapshot.data);
                    }

                    debug!(
                        source = S::NAME,
                        scope = %scope,
                        path = %entry.path.display(),
                        "source cache is stale; refreshing"
                    );
                    // Keep stale cache around as a fallback in case refresh fails.
                    stale_cache = Some(snapshot);
                    break;
                }
                Err(error) => {
                    warn!(
                        source = S::NAME,
                        scope = %scope,
                        path = %entry.path.display(),
                        error = %error,
                        "failed to read source cache snapshot"
                    );
                }
            }
        }

        // Refresh the source when there is no cache yet or the cache is stale.
        match fetch(stale_cache.clone()).await {
            Ok(data) => {
                Self::write_cache(&cache_dir, now, &data)?;
                debug!(
                    source = S::NAME,
                    scope = %scope,
                    structures = data.data.len(),
                    "refreshed source cache"
                );
                Ok(data)
            }
            Err(error) => match stale_cache {
                Some(cached) => {
                    warn!(
                        source = S::NAME,
                        scope = %scope,
                        error = %error,
                        "source refresh failed; serving stale cache"
                    );

                    if let Err(write_error) = Self::write_cache(&cache_dir, now, &cached.data) {
                        warn!(
                            source = S::NAME,
                            scope = %scope,
                            error = %write_error,
                            "failed to refresh stale cache timestamp after source refresh error"
                        );
                    }

                    Ok(cached.data)
                }
                None => {
                    warn!(
                        source = S::NAME,
                        scope = %scope,
                        error = %error,
                        "source refresh failed without usable cache"
                    );
                    Err(error)
                }
            },
        }
    }

    fn cache_dir() -> PathBuf {
        PathBuf::from("data").join(S::NAME)
    }

    fn scoped_cache_dir(scope: Scope) -> PathBuf {
        let cache_key = match S::CACHE_VERSION {
            "v1" => scope.cache_key(),
            version => format!("{}-{version}", scope.cache_key()),
        };

        Self::cache_dir().join(cache_key)
    }

    fn ensure_cache_dir(scope: Scope) -> Result<PathBuf, DynError> {
        let cache_dir = Self::scoped_cache_dir(scope);
        std::fs::create_dir_all(&cache_dir)?;
        Ok(cache_dir)
    }

    fn cache_entries(cache_dir: &Path) -> Result<Vec<CacheEntry>, DynError> {
        let mut entries = Vec::new();

        for entry in std::fs::read_dir(cache_dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if !metadata.is_file() {
                continue;
            }

            entries.push(CacheEntry {
                path: entry.path(),
                modified: metadata.modified()?,
            });
        }

        entries.sort_by(|left, right| right.modified.cmp(&left.modified));
        Ok(entries)
    }

    fn read_cache(path: &Path) -> Result<DataCollection, DynError> {
        let file_bytes = std::fs::read(path)?;
        Ok(postcard::from_bytes(&file_bytes)?)
    }

    fn write_cache(
        cache_dir: &Path,
        now: SystemTime,
        data: &DataCollection,
    ) -> Result<(), DynError> {
        // Use a timestamped filename so older snapshots can coexist until cleanup is added.
        let timestamp = now.duration_since(UNIX_EPOCH)?.as_nanos();
        let file_name = format!("{timestamp}.bin");
        let final_path = cache_dir.join(file_name);
        let temp_path = final_path.with_extension("bin.tmp");
        let encoded = postcard::to_extend(data, Vec::new())?;

        // Write to a temp file first so readers never observe a partially written cache file.
        std::fs::write(&temp_path, encoded)?;
        std::fs::rename(temp_path, final_path)?;

        Ok(())
    }

    fn is_fresh(modified: SystemTime, now: SystemTime) -> bool {
        match now.duration_since(modified) {
            Ok(age) => age <= REFRESH_INTERVAL,
            Err(_) => true,
        }
    }
}

impl<S: Source> Default for StorageWrapper<S> {
    fn default() -> Self {
        Self::new()
    }
}
