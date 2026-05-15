use std::borrow::Cow;

use crate::poll::response::demographic::{
    education_level::EducationLevel, ethnicity::Ethnicity, ideology::Ideology,
    partisan_affiliation::PartisanAffiliation, sex::Sex,
};

pub mod education_level;
pub mod ethnicity;
pub mod ideology;
pub mod partisan_affiliation;
pub mod sex;

#[derive(Debug, Clone)]
pub enum Demographic {
    All,
    Age {
        lower_bound: u8,
        upper_bound: u8,
    },
    Sex {
        sex: Sex,
    },
    Ethnicity {
        ethnicity: Ethnicity,
    },
    EducationLevel {
        education_level: EducationLevel,
    },
    VoterRegistration {
        regeristered: bool,
    },
    Ideology {
        ideology: Ideology,
    },
    PartisanAffiliation {
        partisan_affiliation: PartisanAffiliation,
    },
    Other {
        description: Cow<'static, str>,
    },
}

pub fn demographic_key(demographic: &Demographic) -> String {
    match demographic {
        Demographic::All => "all".to_string(),
        Demographic::Age {
            lower_bound,
            upper_bound,
        } => {
            format!("age:{lower_bound}:{upper_bound}")
        }
        Demographic::Sex { sex } => format!("sex:{}", sex_label(sex)),
        Demographic::Ethnicity { ethnicity } => {
            format!("ethnicity:{}", ethnicity_label(ethnicity))
        }
        Demographic::EducationLevel { education_level } => {
            format!("education_level:{}", education_level_label(education_level))
        }
        Demographic::VoterRegistration { regeristered } => {
            format!("voter_registration:{regeristered}")
        }
        Demographic::Ideology { ideology } => format!("ideology:{}", ideology_label(ideology)),
        Demographic::PartisanAffiliation {
            partisan_affiliation,
        } => format!(
            "partisan_affiliation:{}",
            partisan_affiliation_label(partisan_affiliation)
        ),
        Demographic::Other { description } => format!("other:{}", description.as_ref()),
    }
}

fn sex_label(sex: &Sex) -> &'static str {
    match sex {
        Sex::Other => "other",
        Sex::Male => "male",
        Sex::Female => "female",
    }
}

fn ethnicity_label(ethnicity: &Ethnicity) -> &'static str {
    match ethnicity {
        Ethnicity::Other => "other",
        Ethnicity::White => "white",
        Ethnicity::Black => "black",
        Ethnicity::Asian => "asian",
        Ethnicity::Hispanic => "hispanic",
    }
}

fn education_level_label(education_level: &EducationLevel) -> &'static str {
    match education_level {
        EducationLevel::Other => "other",
        EducationLevel::NoDegree => "no_degree",
        EducationLevel::HighSchool => "high_school",
        EducationLevel::CollegeGrad => "college_grad",
        EducationLevel::Bachelors => "bachelors",
        EducationLevel::Masters => "masters",
        EducationLevel::Doctorate => "doctorate",
    }
}

fn ideology_label(ideology: &Ideology) -> &'static str {
    match ideology {
        Ideology::Other => "other",
        Ideology::Liberal => "liberal",
        Ideology::Independent => "independent",
        Ideology::Conservative => "conservative",
        Ideology::Moderate => "moderate",
    }
}

fn partisan_affiliation_label(partisan_affiliation: &PartisanAffiliation) -> &'static str {
    match partisan_affiliation {
        PartisanAffiliation::Other => "other",
        PartisanAffiliation::Democrat => "democrat",
        PartisanAffiliation::Republican => "republican",
        PartisanAffiliation::Independent => "independent",
        PartisanAffiliation::Moderate => "moderate",
    }
}
