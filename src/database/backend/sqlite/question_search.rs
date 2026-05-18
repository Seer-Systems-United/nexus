use diesel::{
    RunQueryDsl, SqliteConnection, sql_query,
    sql_types::{BigInt, Text},
};
use tracing::{debug, instrument};

use crate::expr::ExpressionError;

#[derive(diesel::QueryableByName)]
struct QuestionIdRow {
    #[diesel(sql_type = Text)]
    id: String,
}

#[derive(diesel::QueryableByName)]
struct RowId {
    #[diesel(sql_type = BigInt)]
    rowid: i64,
}

#[instrument(skip(conn, text, keywords))]
pub(super) fn upsert_question_fts(
    conn: &mut SqliteConnection,
    question_id: &str,
    text: &str,
    keywords: &str,
) -> Result<(), ExpressionError> {
    let rowid = sql_query("SELECT rowid FROM questions WHERE id = ?")
        .bind::<Text, _>(question_id)
        .get_result::<RowId>(conn)?
        .rowid;

    sql_query("DELETE FROM questions_fts WHERE rowid = ?")
        .bind::<BigInt, _>(rowid)
        .execute(conn)?;

    sql_query("INSERT INTO questions_fts(rowid, id, text, keywords) VALUES (?, ?, ?, ?)")
        .bind::<BigInt, _>(rowid)
        .bind::<Text, _>(question_id)
        .bind::<Text, _>(text)
        .bind::<Text, _>(keywords)
        .execute(conn)?;

    Ok(())
}

#[instrument(skip(conn))]
pub(super) fn question_ids_for_search(
    conn: &mut SqliteConnection,
    question: &str,
) -> Result<Vec<String>, ExpressionError> {
    let Some(query) = plain_text_fts_query(question) else {
        debug!("empty question search after tokenization");
        return Ok(Vec::new());
    };

    sql_query("SELECT id FROM questions_fts WHERE questions_fts MATCH ? ORDER BY rank")
        .bind::<Text, _>(query)
        .load::<QuestionIdRow>(conn)
        .map(|rows| rows.into_iter().map(|row| row.id).collect())
        .map_err(ExpressionError::from)
}

fn plain_text_fts_query(input: &str) -> Option<String> {
    let terms: Vec<String> = input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|term| !term.is_empty())
        .map(|term| format!("\"{}\"", term.replace('"', "\"\"")))
        .collect();

    if terms.is_empty() {
        None
    } else {
        Some(terms.join(" AND "))
    }
}
