pub mod database;
pub mod nlp;
pub mod schema;
pub mod utils;

use crate::{
    database::init_database,
    nlp::{
        init_nlp,
        keywords::extract_keywords,
        subjects::{SubjectType, extract_subjects},
    },
    utils::logging::init_tracing,
};

fn main() {
    init_tracing();
    init_nlp();
    init_database();

    // Example text to extract keywords from
    let text = "5. Should Trump Have Sought Congressional Approval Before Strikes in Iran";

    extract_keywords(text);
    let sub = extract_subjects(&text);

    dbg!(sub);

    let text = "Favorability of Trump Administration Figures — Kristi Noem";

    extract_keywords(text);
    let sub = extract_subjects(&text);

    for sub in &sub {
        if sub.subject_type == SubjectType::Person {
            let name = human_name::Name::parse(&sub.text).unwrap();
            dbg!(name.given_name().unwrap());
        }
    }

    dbg!(sub);
}
