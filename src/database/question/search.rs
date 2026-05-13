use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel_full_text_search::{TsVectorExtensions, plainto_tsquery};
use tracing::{debug, error};

use crate::{
    database::{get_connection, question::DatabaseQuestion},
    poll::question::is_non_question_text,
    schema::{self},
};

pub fn search_questions_by_text(
    question_text: &str,
) -> Result<Vec<DatabaseQuestion>, diesel::result::Error> {
    debug!(question_text = %question_text, "searching questions by text");

    let mut conn = get_connection();

    match schema::questions::table
        .filter(schema::questions::text.eq(question_text))
        .select(DatabaseQuestion::as_select())
        .load::<DatabaseQuestion>(&mut conn)
    {
        Ok(questions) => {
            let questions = filter_non_questions(questions);
            debug!(count = questions.len(), question_text = %question_text, "found questions by text");
            Ok(questions)
        }
        Err(e) => {
            error!(error = %e, question_text = %question_text, "error searching questions by text");
            Err(e)
        }
    }
}

pub fn search_questions_by_keywords(
    keywords: &str,
) -> Result<Vec<DatabaseQuestion>, diesel::result::Error> {
    debug!(keywords = %keywords, "searching questions by keywords");

    let mut conn = get_connection();

    match schema::questions::table
        .filter(schema::questions::keywords.matches(plainto_tsquery(keywords)))
        .select(DatabaseQuestion::as_select())
        .load::<DatabaseQuestion>(&mut conn)
    {
        Ok(questions) => {
            let questions = filter_non_questions(questions);
            debug!(count = questions.len(), keywords = %keywords, "found questions by keywords");
            Ok(questions)
        }
        Err(e) => {
            error!(error = %e, keywords = %keywords, "error searching questions by keywords");
            Err(e)
        }
    }
}

fn filter_non_questions(questions: Vec<DatabaseQuestion>) -> Vec<DatabaseQuestion> {
    questions
        .into_iter()
        .filter(|question| !is_non_question_text(&question.text))
        .collect()
}
