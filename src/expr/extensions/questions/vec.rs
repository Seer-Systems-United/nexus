use crate::database::response::DatabaseResponse;
use crate::default_backend;
use crate::{
    database::{poll::DatabasePoll, question::DatabaseQuestion},
    expr::{extensions::questions::DatabaseQuestionExt, get::get},
};
use std::collections::HashSet;
use tracing::{error, info, instrument};

impl DatabaseQuestionExt for Vec<DatabaseQuestion> {
    #[instrument(skip(self))]
    fn get_questions_text(&self) -> Vec<String> {
        info!(
            "Extracting unique question texts from a list of {} questions",
            self.len()
        );
        let mut ids: HashSet<uuid::Uuid> = HashSet::new();
        let mut texts: Vec<String> = Vec::new();

        for question in self {
            if ids.insert(question.id) {
                texts.push(question.text.clone());
            }
        }

        texts
    }

    #[instrument(skip(self))]
    fn get_polls(&self) -> Vec<DatabasePoll> {
        info!("Fetching unique polls for {} questions", self.len());
        let poll_ids: HashSet<uuid::Uuid> = self.iter().map(|question| question.poll_id).collect();

        // Get polls
        let backend = default_backend().unwrap_or_else(|err| {
            error!(error = ?err, "Failed to retrieve default backend");
            panic!("Failed to retrieve default backend");
        });

        let polls = get()
            .polls()
            .by_ids(poll_ids)
            .execute_with(&backend)
            .unwrap_or_else(|err| {
                error!(error = ?err, "Failed to execute poll fetch");
                panic!("Failed to execute poll fetch: {:?}", err);
            });

        polls
    }

    #[instrument(skip(self))]
    fn get_responses(&self) -> Vec<Vec<DatabaseResponse>> {
        info!("Fetching responses for {} questions", self.len());
        let mut responses: Vec<Vec<DatabaseResponse>> = Vec::new();

        let backend = default_backend().unwrap_or_else(|err| {
            error!(error = ?err, "Failed to retrieve default backend");
            panic!("Failed to retrieve default backend: {:?}", err);
        });

        for question in self {
            let question_responses = get()
                .responses()
                .from_question_id(question.id)
                .execute_with(&backend)
                .unwrap_or_else(|err| {
                    error!(error = ?err, "Failed to execute response fetch");
                    panic!("Failed to execute response fetch: {:?}", err);
                });

            responses.push(question_responses);
        }

        responses
    }
}
