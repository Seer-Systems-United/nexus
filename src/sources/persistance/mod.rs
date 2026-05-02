mod cache;
mod refresh;

pub use cache::CacheSnapshot;

use crate::sources::{DataCollection, Scope, Source};
use cache::SourceCache;
use std::{future::Future, marker::PhantomData, time::SystemTime};
use tracing::debug;

type DynError = Box<dyn std::error::Error + Send + Sync>;

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
        let cache = SourceCache::<S>::open(scope)?;
        let now = SystemTime::now();
        let mut stale_cache = None;

        debug!(
            source = S::NAME,
            scope = %scope,
            cache_dir = %cache.dir().display(),
            "opened source cache directory"
        );

        if let Some(snapshot) = cache.latest_snapshot(now)? {
            if snapshot.fresh {
                return Ok(snapshot.cache.data);
            }
            stale_cache = Some(snapshot.cache);
        }

        match fetch(stale_cache.clone()).await {
            Ok(data) => self.store_fresh_data(&cache, now, scope, data),
            Err(error) => refresh::stale_or_error::<S>(&cache, now, scope, stale_cache, error),
        }
    }

    fn store_fresh_data(
        &self,
        cache: &SourceCache<S>,
        now: SystemTime,
        scope: Scope,
        data: DataCollection,
    ) -> Result<DataCollection, DynError> {
        cache.write(now, &data)?;
        debug!(
            source = S::NAME,
            scope = %scope,
            structures = data.data.len(),
            "refreshed source cache"
        );
        Ok(data)
    }
}

impl<S: Source> Default for StorageWrapper<S> {
    fn default() -> Self {
        Self::new()
    }
}
