use crate::poll::response::{
    demographic::{Demographic, demographic_key},
    unit::Unit,
};

pub(crate) struct DemographicRecord {
    pub(crate) key: String,
    pub(crate) demographic_type: &'static str,
    pub(crate) label: Option<String>,
    pub(crate) lower_bound: Option<i32>,
    pub(crate) upper_bound: Option<i32>,
    pub(crate) registered: Option<bool>,
}

pub(crate) fn demographic_record(demographic: &Demographic) -> DemographicRecord {
    match demographic {
        Demographic::All => DemographicRecord {
            key: demographic_key(demographic),
            demographic_type: "all",
            label: Some("All".to_string()),
            lower_bound: None,
            upper_bound: None,
            registered: None,
        },
        Demographic::Age {
            lower_bound,
            upper_bound,
        } => DemographicRecord {
            key: demographic_key(demographic),
            demographic_type: "age",
            label: Some(format!("{lower_bound}-{upper_bound}")),
            lower_bound: Some(i32::from(*lower_bound)),
            upper_bound: Some(i32::from(*upper_bound)),
            registered: None,
        },
        Demographic::Sex { sex } => DemographicRecord {
            key: demographic_key(demographic),
            demographic_type: "sex",
            label: Some(format!("{sex:?}").to_lowercase()),
            lower_bound: None,
            upper_bound: None,
            registered: None,
        },
        Demographic::Ethnicity { ethnicity } => DemographicRecord {
            key: demographic_key(demographic),
            demographic_type: "ethnicity",
            label: Some(format!("{ethnicity:?}").to_lowercase()),
            lower_bound: None,
            upper_bound: None,
            registered: None,
        },
        Demographic::EducationLevel { education_level } => DemographicRecord {
            key: demographic_key(demographic),
            demographic_type: "education_level",
            label: Some(format!("{education_level:?}").to_lowercase()),
            lower_bound: None,
            upper_bound: None,
            registered: None,
        },
        Demographic::VoterRegistration { regeristered } => DemographicRecord {
            key: demographic_key(demographic),
            demographic_type: "voter_registration",
            label: Some(regeristered.to_string()),
            lower_bound: None,
            upper_bound: None,
            registered: Some(*regeristered),
        },
        Demographic::Ideology { ideology } => DemographicRecord {
            key: demographic_key(demographic),
            demographic_type: "ideology",
            label: Some(format!("{ideology:?}").to_lowercase()),
            lower_bound: None,
            upper_bound: None,
            registered: None,
        },
        Demographic::PartisanAffiliation {
            partisan_affiliation,
        } => DemographicRecord {
            key: demographic_key(demographic),
            demographic_type: "partisan_affiliation",
            label: Some(format!("{partisan_affiliation:?}").to_lowercase()),
            lower_bound: None,
            upper_bound: None,
            registered: None,
        },
        Demographic::Other { description } => DemographicRecord {
            key: demographic_key(demographic),
            demographic_type: "other",
            label: Some(description.to_string()),
            lower_bound: None,
            upper_bound: None,
            registered: None,
        },
    }
}

pub(crate) fn unit_name(unit: &Unit) -> String {
    match unit {
        Unit::Other(name) => name.clone(),
        Unit::Percent => "percent".to_string(),
        Unit::Count => "count".to_string(),
    }
}
