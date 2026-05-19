use crate::database::person::DatabasePerson;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, People},
};

impl NexusExpression<GetOp, People, DatabasePerson> {
    pub fn by_id(self, person_id: uuid::Uuid) -> Self {
        self.push_filter(Filter::PersonId { person_id })
    }
}
