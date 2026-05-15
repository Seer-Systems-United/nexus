pub mod where_as;

use crate::database::person::DatabasePerson;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{People, Table},
};

impl NexusExpression<GetOp> {
    pub fn names(self) -> NexusExpression<GetOp, People, DatabasePerson> {
        self.select_table(Table::People)
    }
}
