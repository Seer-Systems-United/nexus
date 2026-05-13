pub fn extract_pdf_from_url(url: &str) -> Vec<String> {
    let response = reqwest::blocking::get(url).unwrap();
    let pdf_bytes = response.bytes().unwrap().to_vec();
    extract_pdf_pages(pdf_bytes)
}

pub fn extract_pdf_pages(pdf_bytes: Vec<u8>) -> Vec<String> {
    let document = pdf_oxide::PdfDocument::from_bytes(pdf_bytes).unwrap();
    let page_count = document.page_count().unwrap();

    (0..page_count)
        .map(|page_index| document.extract_text(page_index).unwrap())
        .collect()
}
