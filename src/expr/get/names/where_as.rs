use diesel::{ExpressionMethods, QueryDsl};
use tracing::trace;

use crate::database::person::DatabasePerson;
use crate::{
    expr::{
        ExpressionError, NexusExpression,
        common::query::PeopleQuery,
        get::GetOp,
        ops::{Filter, NameField, People},
        traits::{FilterApplication, FilterTrait},
    },
    schema,
};

pub(crate) struct WhereAsFilter;

impl NexusExpression<GetOp, People, DatabasePerson> {
    pub fn where_as(self, field: NameField, value: impl Into<String>) -> Self {
        self.push_filter(Filter::Name {
            field,
            value: value.into(),
        })
    }
}

impl<'a> FilterTrait<PeopleQuery<'a>> for WhereAsFilter {
    fn apply_filter(
        query: PeopleQuery<'a>,
        filter: &Filter,
        _conn: &mut diesel::PgConnection,
    ) -> Result<FilterApplication<PeopleQuery<'a>>, ExpressionError> {
        match filter {
            Filter::Name {
                field: NameField::FirstName,
                value,
            } => {
                trace!(value = %value, "filtering by given_name");
                Ok(FilterApplication::Applied(
                    query.filter(schema::people::given_name.eq(value.clone())),
                ))
            }
            Filter::Name {
                field: NameField::Surname,
                value,
            } => {
                trace!(value = %value, "filtering by surname");
                Ok(FilterApplication::Applied(
                    query.filter(schema::people::surname.eq(value.clone())),
                ))
            }
            _ => Ok(FilterApplication::Skipped(query)),
        }
    }
}
