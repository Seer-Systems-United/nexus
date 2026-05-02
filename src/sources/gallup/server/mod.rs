//! # Gallup server module
//!
//! Orchestrates downloading and extracting Gallup poll articles and charts.
//! Implements the `Source` trait for Gallup.

use crate::sources::{DataCollection, Scope, Source, persistance::StorageWrapper};
use std::io::{Error as IoError, ErrorKind};

pub mod download;
pub mod extract;

/// A chart asset downloaded from Gallup (CSV data).
///
/// # Fields
/// - `title`: The chart title.
/// - `csv_bytes`: Raw CSV file bytes.
#[derive(Debug, Clone)]
pub(crate) struct GallupChartAsset {
    pub title: String,
    pub csv_bytes: Vec<u8>,
}

/// A Gallup article with embedded charts and optional PDF.
///
/// # Fields
/// - `title`: Article title.
/// - `published_on`: Publication date string.
/// - `pdf_bytes`: Optional PDF bytes of the full article.
/// - `charts`: List of charts extracted from the article.
#[derive(Debug, Clone)]
pub(crate) struct GallupArticleAsset {
    pub title: String,
    pub published_on: String,
    pub pdf_bytes: Option<Vec<u8>>,
    pub charts: Vec<GallupChartAsset>,
}

/// Load Gallup data for the given scope, with caching.
///
/// # Parameters
/// - `scope`: The query scope (latest, last N days, etc.).
///
/// # Returns
/// - `Ok(DataCollection)`: The loaded and extracted poll data.
///
/// # Errors
/// - Returns an error if download and extraction fail and no cache is available.
async fn load_gallup(
    scope: Scope,
) -> Result<DataCollection, Box<dyn std::error::Error + Send + Sync>> {
    let storage = StorageWrapper::<super::Gallup>::new();

    storage
        .get_data_with_cache(scope, |cached| async move {
            let articles = download::download_gallup_articles(scope).await?;

            if articles.is_empty() {
                return cached.map(|snapshot| snapshot.data).ok_or_else(|| {
                    IoError::new(
                        ErrorKind::NotFound,
                        format!("Gallup articles not found for scope {scope}"),
                    )
                    .into()
                });
            }

            extract::extract_gallup_data(&articles, scope)
        })
        .await
}

#[async_trait::async_trait]
impl Source for super::Gallup {
    const NAME: &'static str = "Gallup";

    async fn get_data(
        scope: Scope,
    ) -> Result<super::super::DataCollection, Box<dyn std::error::Error + Send + Sync>> {
        load_gallup(scope).await
    }
}
