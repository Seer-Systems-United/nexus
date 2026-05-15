use crate::{
    database::{BackendTrait, DefaultBackend, default_backend},
    expr::ExpressionError,
    expr::ops::PollSourceFilter,
    poll::source::{traits::PollSource, yougov::YouGov},
};
use tracing::warn;

pub mod database;
pub mod expr;
pub mod nlp;
pub mod poll;
pub mod utils;

pub struct Nexus<B = DefaultBackend> {
    backend: B,
}

impl Nexus<DefaultBackend> {
    pub fn new() -> Result<Self, ExpressionError> {
        let backend = default_backend()?;
        Self::with_backend(backend)
    }

    pub fn force_update() -> Result<(), ExpressionError> {
        let backend = default_backend()?;
        Self::from_backend(backend).update_source::<YouGov>()
    }
}

impl<B: BackendTrait> Nexus<B> {
    pub fn with_backend(backend: B) -> Result<Self, ExpressionError> {
        let nexus = Self { backend };
        nexus.update_source::<YouGov>()?;
        Ok(nexus)
    }

    pub fn from_backend(backend: B) -> Self {
        Self { backend }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn update_source<S>(&self) -> Result<(), ExpressionError>
    where
        S: PollSource + PollSourceFilter,
    {
        let poll = S::get_latest_poll();
        if poll.questions.is_empty() {
            warn!(
                source_name = <S as PollSourceFilter>::SOURCE_NAME,
                "latest poll had no parsed questions; skipping database save"
            );
            return Ok(());
        }

        self.backend
            .save_poll(<S as PollSourceFilter>::SOURCE_NAME, &poll)
    }
}
