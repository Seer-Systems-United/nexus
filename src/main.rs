pub mod database;
pub mod nlp;
pub mod poll;
pub mod schema;
pub mod utils;

use crate::{
    database::{
        init_database,
        poll::create::create_poll_in_db,
        question::{create::create_question_in_db, search::search_questions_by_keywords},
        response::create::create_response_in_db,
        source::create::create_source_in_db,
    },
    nlp::init_nlp,
    poll::source::{traits::PollSource, yougov::YouGov},
    utils::logging::init_tracing,
};

fn main() {
    init_tracing();
    init_nlp();
    init_database();

    let poll = YouGov::get_latest_poll();

    dbg!(&poll);

    let source = match create_source_in_db(YouGov::SOURCE_NAME) {
        Ok(source) => source,
        Err(error) => {
            tracing::error!(%error, source = YouGov::SOURCE_NAME, "failed to persist source");
            return;
        }
    };

    let database_poll = match create_poll_in_db(source.id, poll.published_timestamp) {
        Ok(database_poll) => database_poll,
        Err(error) => {
            tracing::error!(%error, source_id = %source.id, "failed to persist poll");
            return;
        }
    };

    for question in &poll.questions {
        match create_question_in_db(database_poll.id, &question.text) {
            Ok(Some(database_question)) => {
                for response in &question.responses {
                    if let Err(error) = create_response_in_db(database_question.id, response) {
                        tracing::error!(
                            %error,
                            question_id = %database_question.id,
                            answer = %response.answer,
                            "failed to persist response"
                        );
                    }
                }
            }
            Ok(None) => {}
            Err(error) => {
                tracing::error!(%error, question = %question.text, "failed to persist question");
            }
        }
    }

    //let question = Question::new(text);
    let resp = search_questions_by_keywords("Trump");

    let _ = dbg!(resp);
}
