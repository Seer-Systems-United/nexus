//! # Emerson server module
//!
//! Orchestrates downloading and extracting Emerson poll workbooks.
//! Implements the `Source` trait for Emerson.

use crate::sources::{DataCollection, Scope, Source, persistance::StorageWrapper};
use std::io::{Error as IoError, ErrorKind};

pub mod download;
pub mod extract;

/// An Emerson workbook downloaded from Google Sheets.
///
/// # Fields
/// - `title`: The title of the poll/workbook.
/// - `date`: The publication date string.
/// - `bytes`: Raw xlsx file bytes.
#[derive(Debug, Clone)]
pub(crate) struct EmersonWorkbook {
    pub title: String,
    pub date: String,
    pub bytes: Vec<u8>,
}

/// Load Emerson data for the given scope, with caching.
///
/// # Parameters
/// - `scope`: The query scope (latest, last N days, etc.).
///
/// # Returns
/// - `Ok(DataCollection)`: The loaded and extracted poll data.
///
/// # Errors
/// - Returns an error if download and extraction fail and no cache is available.
async fn load_emerson(
    scope: Scope,
) -> Result<DataCollection, Box<dyn std::error::Error + Send + Sync>> {
    let storage = StorageWrapper::<super::Emerson>::new();

    storage
        .get_data_with_cache(scope, |cached| async move {
            let workbooks = download::download_emerson_data(scope).await?;

            if workbooks.is_empty() {
                return cached.map(|snapshot| snapshot.data).ok_or_else(|| {
                    IoError::new(
                        ErrorKind::NotFound,
                        format!("Emerson workbook not found for scope {scope}"),
                    )
                    .into()
                });
            }

            extract::extract_emerson_data(&workbooks, scope)
        })
        .await
}

#[async_trait::async_trait]
impl Source for super::Emerson {
    const NAME: &'static str = "Emerson";
    const CACHE_VERSION: &'static str = "v2";

    async fn get_data(
        scope: Scope,
    ) -> Result<super::super::DataCollection, Box<dyn std::error::Error + Send + Sync>> {
        load_emerson(scope).await
    }
}
