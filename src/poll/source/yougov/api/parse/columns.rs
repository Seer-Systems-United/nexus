use std::borrow::Cow;

use crate::poll::response::demographic::{
    Demographic, education_level::EducationLevel, ethnicity::Ethnicity, ideology::Ideology,
    partisan_affiliation::PartisanAffiliation, sex::Sex,
};

#[derive(PartialEq)]
pub(super) enum ColumnSpec {
    Total,
    Male,
    Female,
    White,
    Black,
    Asian,
    Hispanic,
    Age18To29,
    Age30To44,
    Age45To64,
    Age65Plus,
    NoDegree,
    CollegeGrad,
    Voters,
    Lib,
    Mod,
    Con,
    Dem,
    Ind,
    Rep,
    Harris,
    Trump,
    Supporter,
    Other(String),
}

impl ColumnSpec {
    fn from_label(label: &str) -> Self {
        match label {
            "Total" => Self::Total,
            "Male" => Self::Male,
            "Female" => Self::Female,
            "White" => Self::White,
            "Black" => Self::Black,
            "Asian" => Self::Asian,
            "Hispanic" => Self::Hispanic,
            "18-29" => Self::Age18To29,
            "30-44" => Self::Age30To44,
            "45-64" => Self::Age45To64,
            "65+" => Self::Age65Plus,
            "Voters" => Self::Voters,
            "Lib" => Self::Lib,
            "Mod" => Self::Mod,
            "Con" => Self::Con,
            "Dem" => Self::Dem,
            "Ind" => Self::Ind,
            "Rep" => Self::Rep,
            "Harris" => Self::Harris,
            "Trump" => Self::Trump,
            "Supporter" => Self::Supporter,
            _ => Self::Other(label.to_string()),
        }
    }
}

pub(super) fn parse_column_specs(line: &str) -> Option<Vec<ColumnSpec>> {
    let mut tokens = line.split_whitespace().peekable();
    if tokens.next()? != "Total" {
        return None;
    }

    let mut labels = Vec::with_capacity(12);
    labels.push(ColumnSpec::Total);

    while let Some(token) = tokens.next() {
        if token == "No" && tokens.peek().copied() == Some("degree") {
            labels.push(ColumnSpec::NoDegree);
            tokens.next();
        } else if token == "College" && tokens.peek().copied() == Some("grad") {
            labels.push(ColumnSpec::CollegeGrad);
            tokens.next();
        } else {
            labels.push(ColumnSpec::from_label(token));
        }
    }

    if matches!(
        labels.as_slice(),
        [ColumnSpec::Total, ColumnSpec::Harris, ColumnSpec::Trump]
    ) {
        return Some(expanded_vote_columns());
    }

    Some(labels)
}

fn expanded_vote_columns() -> Vec<ColumnSpec> {
    vec![
        ColumnSpec::Total,
        ColumnSpec::Harris,
        ColumnSpec::Trump,
        ColumnSpec::Voters,
        ColumnSpec::Lib,
        ColumnSpec::Mod,
        ColumnSpec::Con,
        ColumnSpec::Supporter,
        ColumnSpec::Dem,
        ColumnSpec::Ind,
        ColumnSpec::Rep,
    ]
}

pub(super) fn demographic_for_column(column: &ColumnSpec) -> Demographic {
    match column {
        ColumnSpec::Total => Demographic::All,
        ColumnSpec::Male => Demographic::Sex { sex: Sex::Male },
        ColumnSpec::Female => Demographic::Sex { sex: Sex::Female },
        ColumnSpec::White => Demographic::Ethnicity {
            ethnicity: Ethnicity::White,
        },
        ColumnSpec::Black => Demographic::Ethnicity {
            ethnicity: Ethnicity::Black,
        },
        ColumnSpec::Asian => Demographic::Ethnicity {
            ethnicity: Ethnicity::Asian,
        },
        ColumnSpec::Hispanic => Demographic::Ethnicity {
            ethnicity: Ethnicity::Hispanic,
        },
        ColumnSpec::Age18To29 => Demographic::Age {
            lower_bound: 18,
            upper_bound: 29,
        },
        ColumnSpec::Age30To44 => Demographic::Age {
            lower_bound: 30,
            upper_bound: 44,
        },
        ColumnSpec::Age45To64 => Demographic::Age {
            lower_bound: 45,
            upper_bound: 64,
        },
        ColumnSpec::Age65Plus => Demographic::Age {
            lower_bound: 65,
            upper_bound: u8::MAX,
        },
        ColumnSpec::NoDegree => Demographic::EducationLevel {
            education_level: EducationLevel::NoDegree,
        },
        ColumnSpec::CollegeGrad => Demographic::EducationLevel {
            education_level: EducationLevel::CollegeGrad,
        },
        ColumnSpec::Voters => Demographic::VoterRegistration { regeristered: true },
        ColumnSpec::Lib => Demographic::Ideology {
            ideology: Ideology::Liberal,
        },
        ColumnSpec::Mod => Demographic::Ideology {
            ideology: Ideology::Moderate,
        },
        ColumnSpec::Con => Demographic::Ideology {
            ideology: Ideology::Conservative,
        },
        ColumnSpec::Dem => Demographic::PartisanAffiliation {
            partisan_affiliation: PartisanAffiliation::Democrat,
        },
        ColumnSpec::Ind => Demographic::PartisanAffiliation {
            partisan_affiliation: PartisanAffiliation::Independent,
        },
        ColumnSpec::Rep => Demographic::PartisanAffiliation {
            partisan_affiliation: PartisanAffiliation::Republican,
        },
        ColumnSpec::Harris => Demographic::Other {
            description: Cow::Borrowed("2024 vote: Harris"),
        },
        ColumnSpec::Trump => Demographic::Other {
            description: Cow::Borrowed("2024 vote: Trump"),
        },
        ColumnSpec::Supporter => Demographic::Other {
            description: Cow::Borrowed("MAGA supporter"),
        },
        ColumnSpec::Other(column) => Demographic::Other {
            description: Cow::Owned(column.clone()),
        },
    }
}
