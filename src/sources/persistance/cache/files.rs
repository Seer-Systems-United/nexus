//! # Cache file operations
//!
//! Handles cache directory management, reading/writing binary cache files
//! using postcard serialization, and freshness checking.

use crate::sources::{DataCollection, Scope, Source};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type DynError = Box<dyn std::error::Error + Send + Sync>;
/// Cache refresh interval: 1 hour.
const REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 60);

/// A cache file entry with its path and modification time.
pub(super) struct CacheEntry {
    pub(super) path: PathBuf,
    pub(super) modified: SystemTime,
}

/// Get the cache directory path for a source and scope.
///
/// # Parameters
/// - `S`: The source type.
/// - `scope`: The query scope.
///
/// # Returns
/// - `PathBuf`: Path like `data/{source_name}/{scope_key}` or with version prefix.
pub(super) fn scoped_cache_dir<S: Source>(scope: Scope) -> PathBuf {
    let cache_key = match S::CACHE_VERSION {
        "v1" => scope.cache_key(),
        version => format!("{}-{version}", scope.cache_key()),
    };

    PathBuf::from("data").join(S::NAME).join(cache_key)
}

/// List all cache file entries in a directory, sorted by most recent first.
///
/// # Parameters
/// - `cache_dir`: The cache directory to scan.
///
/// # Returns
/// - `Ok(Vec<CacheEntry>)`: Sorted list of cache entries.
pub(super) fn cache_entries(cache_dir: &Path) -> Result<Vec<CacheEntry>, DynError> {
    let mut entries = Vec::new();

    for entry in std::fs::read_dir(cache_dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            entries.push(CacheEntry {
                path: entry.path(),
                modified: metadata.modified()?,
            });
        }
    }

    entries.sort_by(|left, right| right.modified.cmp(&left.modified));
    Ok(entries)
}

/// Read and deserialize a cache file.
///
/// # Parameters
/// - `path`: Path to the binary cache file.
///
/// # Returns
/// - `Ok(DataCollection)`: Deserialized data.
pub(super) fn read_cache(path: &Path) -> Result<DataCollection, DynError> {
    let file_bytes = std::fs::read(path)?;
    Ok(postcard::from_bytes(&file_bytes)?)
}

/// Write data to a new cache file atomically (write to temp, then rename).
///
/// # Parameters
/// - `cache_dir`: The cache directory.
/// - `now`: Current system time (used for filename).
/// - `data`: The data to serialize and write.
pub(super) fn write_cache(
    cache_dir: &Path,
    now: SystemTime,
    data: &DataCollection,
) -> Result<(), DynError> {
    let timestamp = now.duration_since(UNIX_EPOCH)?.as_nanos();
    let final_path = cache_dir.join(format!("{timestamp}.bin"));
    let temp_path = final_path.with_extension("bin.tmp");
    let encoded = postcard::to_extend(data, Vec::new())?;

    std::fs::write(&temp_path, encoded)?;
    std::fs::rename(temp_path, final_path)?;
    Ok(())
}

/// Check if a cached file is still fresh based on its modification time.
///
/// # Parameters
/// - `modified`: When the cache file was last modified.
/// - `now`: Current system time.
///
/// # Returns
/// - `true` if the cache is within the refresh interval.
pub(super) fn is_fresh(modified: SystemTime, now: SystemTime) -> bool {
    match now.duration_since(modified) {
        Ok(age) => age <= REFRESH_INTERVAL,
        Err(_) => true,
    }
}
