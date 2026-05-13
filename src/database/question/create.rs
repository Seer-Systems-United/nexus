use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
    SelectableHelper, sql_types::Text,
};
use diesel_full_text_search::to_tsvector;
use tracing::{debug, error, instrument};

use crate::{
    database::{get_connection, question::DatabaseQuestion},
    poll::question::is_non_question_text,
    schema,
};

#[instrument(level = "info", skip_all, fields(poll_id = %poll_id, question_text = %question_text))]
pub fn create_question_in_db(
    poll_id: uuid::Uuid,
    question_text: &str,
) -> Result<Option<DatabaseQuestion>, diesel::result::Error> {
    debug!("creating database_question");

    if is_non_question_text(question_text) {
        debug!("skipping non-question text");
        return Ok(None);
    }

    let mut conn = get_connection();

    match schema::questions::table
        .filter(
            schema::questions::poll_id
                .eq(poll_id)
                .and(schema::questions::text.eq(question_text)),
        )
        .select(DatabaseQuestion::as_select())
        .first::<DatabaseQuestion>(&mut conn)
        .optional()
    {
        Ok(Some(database_question)) => {
            debug!(database_question_id = %database_question.id, "database_question already exists");
            return Ok(Some(database_question));
        }
        Ok(None) => {}
        Err(e) => {
            error!(error = %e, "error checking for existing database_question");
            return Err(e);
        }
    }

    let id = uuid::Uuid::new_v4();

    match diesel::insert_into(schema::questions::table)
        .values((
            schema::questions::id.eq(id),
            schema::questions::poll_id.eq(poll_id),
            schema::questions::text.eq(question_text),
            schema::questions::keywords.eq(to_tsvector::<Text, _>(question_text)),
        ))
        .returning(DatabaseQuestion::as_returning())
        .get_result(&mut conn)
    {
        Ok(database_question) => {
            debug!(database_question_id = %database_question.id, "inserted database_question");
            Ok(Some(database_question))
        }
        Err(e) => {
            error!(error = %e, database_question_id = %id, "error inserting database_question");
            Err(e)
        }
    }
}
