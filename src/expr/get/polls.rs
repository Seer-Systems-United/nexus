use crate::database::poll::DatabasePoll;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, PollSourceFilter, Polls, Table},
};

impl NexusExpression<GetOp> {
    pub fn polls(self) -> NexusExpression<GetOp, Polls, DatabasePoll> {
        self.select_table(Table::Polls)
    }
}

impl NexusExpression<GetOp, Polls, DatabasePoll> {
    pub fn from_source<S: PollSourceFilter>(self, _source: S) -> Self {
        self.push_filter(Filter::PollSource {
            source_name: S::SOURCE_NAME,
        })
    }

    pub fn from_soure<S: PollSourceFilter>(self, source: S) -> Self {
        self.from_source(source)
    }

    pub fn from(self, date: impl Into<String>) -> Self {
        self.push_filter(Filter::PollFrom { date: date.into() })
    }

    pub fn to(self, date: impl Into<String>) -> Self {
        self.push_filter(Filter::PollTo { date: date.into() })
    }
}
