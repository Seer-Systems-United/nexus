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
