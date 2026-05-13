use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{demographic::DatabaseDemographic, get_connection},
    schema,
};

pub fn search_demographics_by_key(
    key: &str,
) -> Result<Vec<DatabaseDemographic>, diesel::result::Error> {
    debug!(demographic_key = %key, "searching demographics by key");

    let mut conn = get_connection();

    match schema::demographics::table
        .filter(schema::demographics::key.eq(key))
        .select(DatabaseDemographic::as_select())
        .load::<DatabaseDemographic>(&mut conn)
    {
        Ok(demographics) => {
            debug!(count = demographics.len(), demographic_key = %key, "found demographics by key");
            Ok(demographics)
        }
        Err(error) => {
            error!(%error, demographic_key = %key, "error searching demographics by key");
            Err(error)
        }
    }
}
