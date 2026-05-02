use crate::sources::{DataCollection, Scope, emerson::server::EmersonWorkbook};
use calamine::Reader;
use std::error::Error;

mod crosstab;
mod topline;
mod utils;

pub use crosstab::parse_crosstab_sheet;

pub(crate) fn extract_emerson_data(
    workbooks: &[EmersonWorkbook],
    _scope: Scope,
) -> Result<DataCollection, Box<dyn Error + Send + Sync>> {
    let mut data = Vec::new();

    for workbook in workbooks {
        let mut wb: calamine::Xlsx<_> =
            calamine::open_workbook_from_rs(std::io::Cursor::new(&workbook.bytes))?;

        if let Ok(range) = wb.worksheet_range(utils::TOPLINE_SHEET_NAME) {
            let rows = range.rows().map(|row| row.to_vec()).collect::<Vec<_>>();
            data.extend(topline::parse_topline_sheet(&rows));
        }

        for sheet_name in [
            utils::CROSSTABS_SHEET_NAME,
            utils::FULL_CROSSTABS_SHEET_NAME,
        ] {
            if let Ok(range) = wb.worksheet_range(sheet_name) {
                let rows = range.rows().map(|row| row.to_vec()).collect::<Vec<_>>();
                data.extend(parse_crosstab_sheet(&rows));
            }
        }
    }

    if data.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "no Emerson poll data found in workbook",
        )));
    }

    let subtitle = workbooks
        .first()
        .map(|wb| format!("{}: {}", wb.title, wb.date));
    Ok(DataCollection {
        title: "Emerson Polls".to_string(),
        subtitle,
        data,
    })
}
