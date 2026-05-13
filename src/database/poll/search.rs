use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{get_connection, poll::DatabasePoll},
    schema,
};

pub fn search_polls_by_source_id(
    source_id: uuid::Uuid,
) -> Result<Vec<DatabasePoll>, diesel::result::Error> {
    debug!(source_id = %source_id, "searching polls by source id");

    let mut conn = get_connection();

    match schema::polls::table
        .filter(schema::polls::source_id.eq(source_id))
        .select(DatabasePoll::as_select())
        .load::<DatabasePoll>(&mut conn)
    {
        Ok(polls) => {
            debug!(count = polls.len(), source_id = %source_id, "found polls by source id");
            Ok(polls)
        }
        Err(error) => {
            error!(%error, source_id = %source_id, "error searching polls by source id");
            Err(error)
        }
    }
}
