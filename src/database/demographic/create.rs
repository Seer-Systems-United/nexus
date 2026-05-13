use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error, instrument};

use crate::{
    database::{demographic::DatabaseDemographic, get_connection},
    poll::response::demographic::{
        Demographic, education_level::EducationLevel, ethnicity::Ethnicity, ideology::Ideology,
        partisan_affiliation::PartisanAffiliation, sex::Sex,
    },
    schema,
};

struct DemographicRecord {
    key: String,
    demographic_type: String,
    label: Option<String>,
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
    registered: Option<bool>,
}

fn create_demographic(record: DemographicRecord) -> DatabaseDemographic {
    DatabaseDemographic {
        id: uuid::Uuid::new_v4(),
        key: record.key,
        demographic_type: record.demographic_type,
        label: record.label,
        lower_bound: record.lower_bound,
        upper_bound: record.upper_bound,
        registered: record.registered,
    }
}

fn demographic_record(demographic: &Demographic) -> DemographicRecord {
    match demographic {
        Demographic::All => DemographicRecord {
            key: "all".to_string(),
            demographic_type: "all".to_string(),
            label: None,
            lower_bound: None,
            upper_bound: None,
            registered: None,
        },
        Demographic::Age {
            lower_bound,
            upper_bound,
        } => DemographicRecord {
            key: format!("age:{lower_bound}:{upper_bound}"),
            demographic_type: "age".to_string(),
            label: None,
            lower_bound: Some(i32::from(*lower_bound)),
            upper_bound: Some(i32::from(*upper_bound)),
            registered: None,
        },
        Demographic::Sex { sex } => labeled_record("sex", sex_label(sex)),
        Demographic::Ethnicity { ethnicity } => {
            labeled_record("ethnicity", ethnicity_label(ethnicity))
        }
        Demographic::EducationLevel { education_level } => {
            labeled_record("education_level", education_level_label(education_level))
        }
        Demographic::VoterRegistration { regeristered } => DemographicRecord {
            key: format!("voter_registration:{regeristered}"),
            demographic_type: "voter_registration".to_string(),
            label: None,
            lower_bound: None,
            upper_bound: None,
            registered: Some(*regeristered),
        },
        Demographic::Ideology { ideology } => labeled_record("ideology", ideology_label(ideology)),
        Demographic::PartisanAffiliation {
            partisan_affiliation,
        } => labeled_record(
            "partisan_affiliation",
            partisan_affiliation_label(partisan_affiliation),
        ),
        Demographic::Other { description } => {
            let description = description.as_ref();
            labeled_record("other", description)
        }
    }
}

fn labeled_record(demographic_type: &str, label: &str) -> DemographicRecord {
    DemographicRecord {
        key: format!("{demographic_type}:{label}"),
        demographic_type: demographic_type.to_string(),
        label: Some(label.to_string()),
        lower_bound: None,
        upper_bound: None,
        registered: None,
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

#[instrument(level = "info", skip_all)]
pub fn create_demographic_in_db(
    demographic: &Demographic,
) -> Result<DatabaseDemographic, diesel::result::Error> {
    debug!("creating demographic");

    let record = demographic_record(demographic);
    let mut conn = get_connection();

    match schema::demographics::table
        .filter(schema::demographics::key.eq(&record.key))
        .select(DatabaseDemographic::as_select())
        .first::<DatabaseDemographic>(&mut conn)
        .optional()
    {
        Ok(Some(demographic)) => {
            debug!(demographic_id = %demographic.id, "demographic already exists");
            return Ok(demographic);
        }
        Ok(None) => {}
        Err(error) => {
            error!(%error, "error checking for existing demographic");
            return Err(error);
        }
    }

    let new_demographic = create_demographic(record);

    match diesel::insert_into(schema::demographics::table)
        .values(&new_demographic)
        .returning(DatabaseDemographic::as_returning())
        .get_result(&mut conn)
    {
        Ok(demographic) => {
            debug!(demographic_id = %demographic.id, "inserted demographic");
            Ok(demographic)
        }
        Err(error) => {
            error!(%error, demographic_id = %new_demographic.id, "error inserting demographic");
            Err(error)
        }
    }
}
