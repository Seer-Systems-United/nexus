//! # Ipsos server module
//!
//! Orchestrates downloading and extracting Ipsos poll PDFs.
//! Implements the `Source` trait for Ipsos.

use crate::sources::{DataCollection, Scope, Source, persistance::StorageWrapper};
use std::io::{Error as IoError, ErrorKind};

pub mod download;
pub mod extract;

/// An Ipsos poll PDF with metadata.
///
/// # Fields
/// - `title`: Poll title.
/// - `published_on`: Publication date string.
/// - `article_url`: URL of the article/page.
/// - `pdf_url`: Direct URL to the PDF.
/// - `bytes`: Raw PDF bytes.
#[derive(Debug, Clone)]
pub(crate) struct IpsosPollPdf {
    pub title: String,
    pub published_on: String,
    pub article_url: String,
    pub pdf_url: String,
    pub bytes: Vec<u8>,
}

/// Load Ipsos data for the given scope, with caching.
///
/// # Parameters
/// - `scope`: The query scope (latest, last N days, etc.).
///
/// # Returns
/// - `Ok(DataCollection)`: The loaded and extracted poll data.
///
/// # Errors
/// - Returns an error if download and extraction fail and no cache is available.
async fn load_ipsos(
    scope: Scope,
) -> Result<DataCollection, Box<dyn std::error::Error + Send + Sync>> {
    let storage = StorageWrapper::<super::Ipsos>::new();

    storage
        .get_data_with_cache(scope, |cached| async move {
            let pdfs = download::download_ipsos_polls(scope).await?;

            if pdfs.is_empty() {
                return cached.map(|snapshot| snapshot.data).ok_or_else(|| {
                    IoError::new(
                        ErrorKind::NotFound,
                        format!("Ipsos poll PDFs not found for scope {scope}"),
                    )
                    .into()
                });
            }

            extract::extract_ipsos_data(&pdfs, scope)
        })
        .await
}

#[async_trait::async_trait]
impl Source for super::Ipsos {
    const NAME: &'static str = "Ipsos";

    async fn get_data(
        scope: Scope,
    ) -> Result<super::super::DataCollection, Box<dyn std::error::Error + Send + Sync>> {
        load_ipsos(scope).await
    }
}
