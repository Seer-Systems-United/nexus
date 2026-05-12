pub mod database;
pub mod nlp;
pub mod poll;
pub mod schema;
pub mod utils;

use crate::{
    database::{init_database, ops::question::search::search_questions_by_keywords},
    nlp::{
        init_nlp,
        keywords::extract_keywords,
        subjects::{SubjectType, extract_subjects},
    },
    poll::question::Question,
    utils::logging::init_tracing,
};

fn main() {
    init_tracing();
    init_nlp();
    init_database();

    // Example text to extract keywords from
    let text = "5. Should Trump Have Sought Congressional Approval Before Strikes in Iran";

    //let question = Question::new(text);
    let resp = search_questions_by_keywords("Trump");

    dbg!(resp);

    let text = "Favorability of Trump Administration Figures — Kristi Noem";
    //let question = Question::new(text);
}
