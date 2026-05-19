use crate::database::person::DatabasePerson;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, People},
};

impl NexusExpression<GetOp, People, DatabasePerson> {
    pub fn by_ids(self, person_ids: impl IntoIterator<Item = uuid::Uuid>) -> Self {
        self.push_filter(Filter::PersonIds {
            person_ids: person_ids.into_iter().collect(),
        })
    }
}
