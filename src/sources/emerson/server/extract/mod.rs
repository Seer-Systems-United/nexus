use crate::sources::{DataCollection, Scope, emerson::server::EmersonWorkbook};
use calamine::Reader;
use std::error::Error;

mod crosstab;
mod topline;
mod utils;

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
                data.extend(crosstab::parse_crosstab_sheet(&rows));
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

#[cfg(test)]
mod tests {
    use super::crosstab::parse_crosstab_sheet;
    use crate::sources::DataStructure;

    fn empty() -> calamine::Data {
        calamine::Data::Empty
    }

    fn text(value: &str) -> calamine::Data {
        calamine::Data::String(value.to_string())
    }

    fn number(value: f64) -> calamine::Data {
        calamine::Data::Float(value)
    }

    #[test]
    fn crosstab_rows_are_answer_options_with_values_across_columns() {
        let rows = vec![
            vec![
                empty(),
                empty(),
                text("Do you approve? - Selected Choice"),
                empty(),
                empty(),
                empty(),
            ],
            vec![
                empty(),
                empty(),
                text("Approve"),
                empty(),
                text("Disapprove"),
                empty(),
            ],
            vec![],
            vec![
                text("Party"),
                text("Democrat"),
                empty(),
                number(0.75),
                empty(),
                number(0.15),
            ],
            vec![
                empty(),
                text("Republican"),
                empty(),
                number(0.20),
                empty(),
                number(0.70),
            ],
            vec![
                text("Race"),
                text("White"),
                empty(),
                number(45.0),
                empty(),
                number(40.0),
            ],
        ];

        let structures = parse_crosstab_sheet(&rows);

        assert_eq!(structures.len(), 1);
        let DataStructure::Crosstab { title, panels, .. } = &structures[0] else {
            panic!("expected crosstab");
        };

        assert_eq!(title, "Do you approve?");
        assert_eq!(panels[0].columns, vec!["Democrat", "Republican", "White"]);
        assert_eq!(panels[0].rows.len(), 2);
        assert_eq!(panels[0].rows[0].label, "Approve");
        assert_eq!(panels[0].rows[0].values, vec![75.0, 20.0, 45.0]);
        assert_eq!(panels[0].rows[1].label, "Disapprove");
        assert_eq!(panels[0].rows[1].values, vec![15.0, 70.0, 40.0]);
    }
}
