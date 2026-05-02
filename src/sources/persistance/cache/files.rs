use crate::sources::{DataCollection, Scope, Source};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type DynError = Box<dyn std::error::Error + Send + Sync>;
const REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 60);

#[derive(Clone)]
pub(super) struct CacheEntry {
    pub(super) path: PathBuf,
    pub(super) modified: SystemTime,
}

pub(super) fn scoped_cache_dir<S: Source>(scope: Scope) -> PathBuf {
    let cache_key = match S::CACHE_VERSION {
        "v1" => scope.cache_key(),
        version => format!("{}-{version}", scope.cache_key()),
    };

    PathBuf::from("data").join(S::NAME).join(cache_key)
}

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

pub(super) fn read_cache(path: &Path) -> Result<DataCollection, DynError> {
    let file_bytes = std::fs::read(path)?;
    Ok(postcard::from_bytes(&file_bytes)?)
}

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

pub(super) fn is_fresh(modified: SystemTime, now: SystemTime) -> bool {
    match now.duration_since(modified) {
        Ok(age) => age <= REFRESH_INTERVAL,
        Err(_) => true,
    }
}
