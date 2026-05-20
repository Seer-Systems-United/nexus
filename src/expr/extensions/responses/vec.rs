use std::collections::HashSet;

use crate::{database::BackendTrait, default_backend};
use crate::{
    database::{
        demographic::DatabaseDemographic, poll::DatabasePoll, question::DatabaseQuestion,
        response::DatabaseResponse, response_unit::DatabaseResponseUnit,
    },
    expr::{
        extensions::{questions::DatabaseQuestionExt, responses::DatabaseResponseExt},
        get::get,
    },
};
use tracing::{error, info, instrument};

impl DatabaseResponseExt for Vec<DatabaseResponse> {
    #[instrument(skip(self))]
    fn get_answers(&self) -> Vec<String> {
        info!(
            "Extracting unique answers from a list of {} responses",
            self.len()
        );
        let mut ids: HashSet<uuid::Uuid> = HashSet::new();
        let mut answers = Vec::new();

        for response in self {
            if ids.insert(response.id) {
                answers.push(response.answer.clone());
            }
        }

        answers
    }

    #[instrument(skip(self))]
    fn get_demographics(&self) -> Vec<DatabaseDemographic> {
        info!("Fetching demographics for {} responses", self.len());
        let demographic_ids: HashSet<uuid::Uuid> = self
            .iter()
            .map(|response| response.demographic_id)
            .collect();

        let backend = default_backend().unwrap_or_else(|err| {
            error!(error = ?err, "Failed to retrieve default backend");
            panic!("Failed to retrieve default backend: {:?}", err);
        });

        backend
            .get_demographics_by_ids(demographic_ids.into_iter().collect())
            .unwrap_or_else(|err| {
                error!(error = ?err, "Failed to execute demographic fetch");
                panic!("Failed to execute demographic fetch: {:?}", err);
            })
    }

    #[instrument(skip(self))]
    fn get_units(&self) -> Vec<DatabaseResponseUnit> {
        info!("Fetching response units for {} responses", self.len());
        let unit_ids: HashSet<uuid::Uuid> = self.iter().map(|response| response.unit_id).collect();

        let backend = default_backend().unwrap_or_else(|err| {
            error!(error = ?err, "Failed to retrieve default backend");
            panic!("Failed to retrieve default backend: {:?}", err);
        });

        backend
            .get_response_units_by_ids(unit_ids.into_iter().collect())
            .unwrap_or_else(|err| {
                error!(error = ?err, "Failed to execute response unit fetch");
                panic!("Failed to execute response unit fetch: {:?}", err);
            })
    }

    #[instrument(skip(self))]
    fn get_questions(&self) -> Vec<DatabaseQuestion> {
        info!("Fetching questions for {} responses", self.len());
        let question_ids: HashSet<uuid::Uuid> =
            self.iter().map(|response| response.question_id).collect();

        let backend = default_backend().unwrap_or_else(|err| {
            error!(error = ?err, "Failed to retrieve default backend");
            panic!("Failed to retrieve default backend: {:?}", err);
        });

        get()
            .questions()
            .by_ids(question_ids)
            .execute_with(&backend)
            .unwrap_or_else(|err| {
                error!(error = ?err, "Failed to execute question fetch");
                panic!("Failed to execute question fetch: {:?}", err);
            })
    }

    #[instrument(skip(self))]
    fn get_polls(&self) -> Vec<DatabasePoll> {
        info!("Fetching polls for {} responses", self.len());
        self.get_questions().get_polls()
    }
}
