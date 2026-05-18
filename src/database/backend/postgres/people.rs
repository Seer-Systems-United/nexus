use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tracing::{debug, trace};

use crate::{
    database::{common::filter::invalid_filter, person::DatabasePerson},
    expr::{
        ExpressionError,
        ops::{Filter, NameField, Table},
    },
};

use super::{connection::get_connection, rows::PersonRow, schema};

pub(super) fn get_people(filters: &[Filter]) -> Result<Vec<DatabasePerson>, ExpressionError> {
    debug!(?filters, "executing Get(People) query");

    let mut conn = get_connection();
    let mut query = schema::people::table.into_boxed();

    for filter in filters {
        match filter {
            Filter::Name {
                field: NameField::FirstName,
                value,
            } => {
                trace!(value = %value, "filtering by given_name");
                query = query.filter(schema::people::given_name.eq(value));
            }
            Filter::Name {
                field: NameField::Surname,
                value,
            } => {
                trace!(value = %value, "filtering by surname");
                query = query.filter(schema::people::surname.eq(value));
            }
            filter => return invalid_filter(Table::People, filter),
        }
    }

    let results = query.load::<PersonRow>(&mut conn)?;
    trace!(count = results.len(), "query returned results");

    Ok(results.into_iter().map(DatabasePerson::from).collect())
}
