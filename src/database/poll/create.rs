use chrono::{DateTime, Utc};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use tracing::{debug, error, instrument};

use crate::{
    database::{get_connection, poll::DatabasePoll},
    schema,
};

fn create_poll(source_id: uuid::Uuid, published_timestamp: chrono::NaiveDateTime) -> DatabasePoll {
    DatabasePoll {
        id: uuid::Uuid::new_v4(),
        source_id,
        published_timestamp,
    }
}

#[instrument(level = "info", skip_all, fields(source_id = %source_id, published_timestamp = %published_timestamp))]
pub fn create_poll_in_db(
    source_id: uuid::Uuid,
    published_timestamp: DateTime<Utc>,
) -> Result<DatabasePoll, diesel::result::Error> {
    debug!("creating poll");

    let mut conn = get_connection();
    let published_timestamp = published_timestamp.naive_utc();

    match schema::polls::table
        .filter(
            schema::polls::source_id
                .eq(source_id)
                .and(schema::polls::published_timestamp.eq(published_timestamp)),
        )
        .select(DatabasePoll::as_select())
        .first::<DatabasePoll>(&mut conn)
        .optional()
    {
        Ok(Some(poll)) => {
            debug!(poll_id = %poll.id, "poll already exists");
            return Ok(poll);
        }
        Ok(None) => {}
        Err(error) => {
            error!(%error, "error checking for existing poll");
            return Err(error);
        }
    }

    let new_poll = create_poll(source_id, published_timestamp);

    match diesel::insert_into(schema::polls::table)
        .values(&new_poll)
        .returning(DatabasePoll::as_returning())
        .get_result(&mut conn)
    {
        Ok(poll) => {
            debug!(poll_id = %poll.id, "inserted poll");
            Ok(poll)
        }
        Err(error) => {
            error!(%error, poll_id = %new_poll.id, "error inserting poll");
            Err(error)
        }
    }
}
