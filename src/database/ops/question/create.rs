use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper, sql_types::Text};
use diesel_full_text_search::to_tsvector;
use tracing::{debug, error, instrument};

use crate::{
    database::{get_connection, ops::question::DatabaseQuestion},
    schema,
};

#[instrument(level = "info", skip_all, fields(question_text = %question_text))]
pub fn create_question_in_db(
    question_text: &str,
) -> Result<DatabaseQuestion, diesel::result::Error> {
    debug!("creating database_question");

    let mut conn = get_connection();

    let id = uuid::Uuid::new_v4();

    match diesel::insert_into(schema::questions::table)
        .values((
            schema::questions::id.eq(id),
            schema::questions::text.eq(question_text),
            schema::questions::keywords.eq(to_tsvector::<Text, _>(question_text)),
        ))
        .returning(DatabaseQuestion::as_returning())
        .get_result(&mut conn)
    {
        Ok(database_question) => {
            debug!(database_question_id = %database_question.id, "inserted database_question");
            Ok(database_question)
        }
        Err(e) => {
            error!(error = %e, database_question_id = %id, "error inserting database_question");
            Err(e)
        }
    }
}
