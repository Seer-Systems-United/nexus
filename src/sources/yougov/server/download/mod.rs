//! # YouGov download module
//!
//! Downloads YouGov poll PDFs from The Economist website.
//! Fetches PDF URLs from article pages and downloads the PDFs.

mod client;
mod links;

pub use links::{clean_pdf_url, is_economist_crosstabs_pdf_url};

use std::error::Error;

type DynError = Box<dyn Error + Send + Sync>;

/// Download YouGov poll PDFs for the given scope.
///
/// Fetches article URLs from The Economist landing page,
/// then downloads the corresponding crosstabs PDFs.
///
/// # Parameters
/// - `scope`: The query scope (determines how many PDFs to download).
///
/// # Returns
/// - `Ok(Vec<Vec<u8>>)`: Downloaded PDF bytes.
///
/// # Errors
/// - Returns an error if no PDF URLs are found.
pub(crate) async fn download_yougov_data(
    scope: crate::sources::Scope,
) -> Result<Vec<Vec<u8>>, DynError> {
    tracing::info!(source = "yougov", "downloading YouGov poll PDFs");
    let pdf_urls = links::poll_pdf_urls(scope).await?;
    let mut all_pdf_bytes = Vec::new();

    for pdf_url in pdf_urls {
        match client::fetch_bytes(&pdf_url).await {
            Ok(pdf_bytes) => {
                tracing::info!(
                    source = "yougov",
                    pdf_url = %pdf_url,
                    bytes = pdf_bytes.len(),
                    "downloaded YouGov poll PDF"
                );
                all_pdf_bytes.push(pdf_bytes);
            }
            Err(error) => {
                tracing::warn!(
                    source = "yougov",
                    error = %error,
                    url = %pdf_url,
                    "failed to download PDF"
                );
            }
        }
    }

    Ok(all_pdf_bytes)
}
