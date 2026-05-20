use tracing::{debug, error, instrument};

#[derive(Debug)]
pub struct GoogleSheetLink {
    pub link: String,
}

impl GoogleSheetLink {
    #[instrument]
    pub fn new(link: String) -> Self {
        debug!(%link, "Creating new GoogleSheetLink");
        Self { link }
    }

    #[instrument(skip(self))]
    pub fn get_as_csv_url(&self, sheet: Option<&str>) -> String {
        let base = self.link.split('/').take(6).collect::<Vec<_>>().join("/");
        let url = match sheet {
            Some(s) => format!(
                "{}/gviz/tq?tqx=out:csv&sheet={}",
                base,
                urlencoding::encode(s)
            ),
            None => format!("{}/gviz/tq?tqx=out:csv", base),
        };
        debug!(%url, ?sheet, "Generated CSV URL");
        url
    }

    #[instrument(skip(self))]
    pub fn get_as_csv(&self, sheet: Option<&str>) -> String {
        let url = self.get_as_csv_url(sheet);
        debug!(%url, ?sheet, "Fetching spreadsheet as CSV");

        match reqwest::blocking::get(&url) {
            Ok(response) => match response.text() {
                Ok(text) => {
                    debug!(
                        size = text.len(),
                        "Successfully retrieved spreadsheet CSV content"
                    );
                    text
                }
                Err(e) => {
                    error!(error = ?e, %url, "Failed to read spreadsheet response text");
                    String::new()
                }
            },
            Err(e) => {
                error!(error = ?e, %url, "Failed to fetch spreadsheet URL");
                String::new()
            }
        }
    }
}
