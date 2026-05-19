use std::collections::HashSet;

use crate::{database::BackendTrait, default_backend};
use crate::{
    database::{poll::DatabasePoll, question::DatabaseQuestion, response::DatabaseResponse},
    expr::{
        extensions::{polls::DatabasePollExt, questions::DatabaseQuestionExt},
        get::get,
    },
};
use tracing::{error, info, instrument};

impl DatabasePollExt for Vec<DatabasePoll> {
    #[instrument(skip(self))]
    fn get_names(&self) -> Vec<String> {
        info!("Fetching source names for {} polls", self.len());
        let source_ids: HashSet<uuid::Uuid> = self.iter().map(|poll| poll.source_id).collect();

        let backend = default_backend().unwrap_or_else(|err| {
            error!(error = ?err, "Failed to retrieve default backend");
            panic!("Failed to retrieve default backend: {:?}", err);
        });

        backend
            .get_source_names_by_ids(source_ids.into_iter().collect())
            .unwrap_or_else(|err| {
                error!(error = ?err, "Failed to execute source name fetch");
                panic!("Failed to execute source name fetch: {:?}", err);
            })
    }

    #[instrument(skip(self))]
    fn get_published_timestamps(&self) -> Vec<chrono::NaiveDateTime> {
        info!(
            "Extracting unique published timestamps from a list of {} polls",
            self.len()
        );
        let mut ids: HashSet<uuid::Uuid> = HashSet::new();
        let mut timestamps = Vec::new();

        for poll in self {
            if ids.insert(poll.id) {
                timestamps.push(poll.published_timestamp);
            }
        }

        timestamps
    }

    #[instrument(skip(self))]
    fn get_questions(&self) -> Vec<Vec<DatabaseQuestion>> {
        info!("Fetching questions for {} polls", self.len());
        let backend = default_backend().unwrap_or_else(|err| {
            error!(error = ?err, "Failed to retrieve default backend");
            panic!("Failed to retrieve default backend: {:?}", err);
        });

        self.iter()
            .map(|poll| {
                get()
                    .questions()
                    .from_poll_id(poll.id)
                    .execute_with(&backend)
                    .unwrap_or_else(|err| {
                        error!(error = ?err, poll_id = %poll.id, "Failed to execute question fetch");
                        panic!("Failed to execute question fetch: {:?}", err);
                    })
            })
            .collect()
    }

    #[instrument(skip(self))]
    fn get_responses(&self) -> Vec<Vec<DatabaseResponse>> {
        info!("Fetching responses for {} polls", self.len());
        self.get_questions()
            .into_iter()
            .map(|questions| questions.get_responses().into_iter().flatten().collect())
            .collect()
    }
}
