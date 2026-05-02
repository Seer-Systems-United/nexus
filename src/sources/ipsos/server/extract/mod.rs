//! # Ipsos extraction module
//!
//! Extracts poll data from Ipsos PDFs by parsing text
//! into bar graphs, crosstabs, and unstructured data.

mod parse;
mod summary;
mod text;

pub use parse::{parse_questions, parse_row};
pub use text::{is_question_title, normalize_line};

use crate::sources::{DataCollection, DataStructure, Scope};
use std::error::Error;

/// Boxed dynamic error type for Ipsos operations.
type DynError = Box<dyn Error + Send + Sync>;

/// Prefix a data structure title with a given string.
fn prefix_structure(structure: &mut DataStructure, prefix: &str) {
    match structure {
        DataStructure::BarGraph { title, .. }
        | DataStructure::LineGraph { title, .. }
        | DataStructure::PieChart { title, .. } => {
            *title = format!("{prefix}: {title}");
        }
        DataStructure::Crosstab { title, prompt, .. } => {
            *title = format!("{prefix}: {title}");
            *prompt = format!("{prefix}: {prompt}");
        }
        DataStructure::Unstructured { data } => {
            *data = format!("{prefix}\n\n{data}");
        }
    }
}

/// Extract Ipsos data from a list of poll PDFs.
///
/// For each PDF, extracts text and parses questions.
/// If no structured data is found, falls back to unstructured text.
///
/// # Parameters
/// - `pdfs`: Downloaded Ipsos poll PDFs.
/// - `scope`: The query scope.
///
/// # Returns
/// - `Ok(DataCollection)`: Combined data from all PDFs.
///
/// # Errors
/// - Returns an error if no valid data is found.
pub(crate) fn extract_ipsos_data(
    pdfs: &[crate::sources::ipsos::server::IpsosPollPdf],
    scope: Scope,
) -> Result<DataCollection, DynError> {
    let mut data = Vec::new();
    let mut pdf_failures = 0usize;
    let mut fallback_count = 0usize;

    for pdf in pdfs {
        let text = match pdf_extract::extract_text_from_mem(&pdf.bytes) {
            Ok(text) => text,
            Err(error) => {
                pdf_failures += 1;
                tracing::warn!(
                    source = "ipsos",
                    article_url = %pdf.article_url,
                    pdf_url = %pdf.pdf_url,
                    error = %error,
                    "failed to extract Ipsos PDF text"
                );
                continue;
            }
        };
        let lines = text
            .lines()
            .map(normalize_line)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();
        let mut structures = parse_questions(&lines);
        let prefix = format!("{}: {}", pdf.published_on, pdf.title);

        if structures.is_empty() {
            fallback_count += 1;
            structures.push(DataStructure::Unstructured { data: text });
        }
        for structure in &mut structures {
            prefix_structure(structure, &prefix);
        }
        data.extend(structures);
    }

    if data.is_empty() {
        return Err("no Ipsos poll data found in PDFs".into());
    }

    tracing::info!(
        source = "ipsos",
        scope = %scope,
        structures = data.len(),
        pdf_failures,
        fallback_count,
        "extracted Ipsos source data"
    );

    Ok(DataCollection {
        title: "Ipsos Polls".to_string(),
        subtitle: summary::collection_subtitle(scope, pdfs),
        data,
    })
}
