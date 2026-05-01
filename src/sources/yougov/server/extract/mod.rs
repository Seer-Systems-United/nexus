use crate::sources::{DataCollection, DataStructure};
use std::error::Error;

mod parse;
mod templates;
mod utils;

pub(crate) fn extract_yougov_data(
    pdfs: &[Vec<u8>],
) -> Result<DataCollection, Box<dyn Error + Send + Sync>> {
    let mut all_data = Vec::new();
    let mut main_title = String::new();
    let mut main_subtitle = None;

    for (index, pdf_bytes) in pdfs.iter().enumerate() {
        let text = match pdf_extract::extract_text_from_mem(pdf_bytes) {
            Ok(text) => text,
            Err(_) => continue,
        };
        let lines = text.lines().map(utils::normalize_line).collect::<Vec<_>>();
        let (title, subtitle) = match extract_document_header(&lines) {
            Ok(header) => header,
            Err(_) => continue,
        };

        if index == 0 {
            main_title = title;
            main_subtitle = subtitle.clone();
        }

        let prefix = subtitle.unwrap_or_else(|| "Unknown Date".to_string());
        let mut data = parse::parse_questions(&lines);
        for structure in &mut data {
            match structure {
                DataStructure::BarGraph { title, .. }
                | DataStructure::LineGraph { title, .. }
                | DataStructure::PieChart { title, .. } => {
                    *title = format!("{}: {}", prefix, title);
                }
                DataStructure::Crosstab { title, prompt, .. } => {
                    *title = format!("{}: {}", prefix, title);
                    *prompt = format!("{}: {}", prefix, prompt);
                }
                DataStructure::Unstructured { .. } => {}
            }
        }
        all_data.extend(data);
    }

    if all_data.is_empty() {
        return Err("no poll questions found in Economist crosstabs PDF".into());
    }
    if main_title.is_empty() {
        main_title = "The Economist/YouGov Poll".to_string();
    }

    tracing::info!(
        source = "yougov",
        structures = all_data.len(),
        "extracted YouGov source data"
    );

    Ok(DataCollection {
        title: main_title,
        subtitle: main_subtitle,
        data: all_data,
    })
}

pub(crate) fn extract_document_header(
    lines: &[String],
) -> Result<(String, Option<String>), Box<dyn Error + Send + Sync>> {
    let mut non_empty = lines
        .iter()
        .filter(|line| !line.is_empty() && !utils::is_page_number(line));
    let title = match non_empty.next() {
        Some(title) => title.clone(),
        None => return Err("missing poll document title".into()),
    };
    let subtitle = non_empty.next().cloned();
    Ok((title, subtitle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_header_extraction_finds_title() {
        let lines = vec![
            String::new(),
            "The Economist/YouGov Poll".to_string(),
            "April 1 - 4, 2026".to_string(),
        ];
        let (title, subtitle) = extract_document_header(&lines).unwrap();
        assert_eq!(title, "The Economist/YouGov Poll");
        assert_eq!(subtitle.as_deref(), Some("April 1 - 4, 2026"));
    }
}
