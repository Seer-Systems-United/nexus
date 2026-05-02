//! # YouGov server module
//!
//! Orchestrates downloading and extracting YouGov poll PDFs.
//! Implements the `Source` trait for YouGov.

use crate::sources::{DataCollection, Scope, Source, persistance::StorageWrapper};

pub mod download;

pub mod extract;

/// Load YouGov data for the given scope, with caching.
///
/// # Parameters
/// - `scope`: The query scope (latest, last N days, etc.).
///
/// # Returns
/// - `Ok(DataCollection)`: The loaded and extracted poll data.
///
/// # Errors
/// - Returns an error if download and extraction fail.
async fn load_yougov(
    scope: Scope,
) -> Result<DataCollection, Box<dyn std::error::Error + Send + Sync>> {
    let storage = StorageWrapper::<super::YouGov>::new();

    storage
        .get_data(scope, || async {
            let pdfs = download::download_yougov_data(scope).await?;

            extract::extract_yougov_data(&pdfs)
        })
        .await
}

#[async_trait::async_trait]
impl super::super::Source for super::YouGov {
    const NAME: &'static str = "YouGov";

    async fn get_data(
        scope: Scope,
    ) -> Result<super::super::DataCollection, Box<dyn std::error::Error + Send + Sync>> {
        load_yougov(scope).await
    }
}
