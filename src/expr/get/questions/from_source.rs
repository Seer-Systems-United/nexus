use crate::database::question::DatabaseQuestion;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, PollSourceFilter, Questions},
};

impl NexusExpression<GetOp, Questions, DatabaseQuestion> {
    pub fn from_source<S: PollSourceFilter>(self, _source: S) -> Self {
        self.push_filter(Filter::QuestionSource {
            source_name: S::SOURCE_NAME,
        })
    }
}
