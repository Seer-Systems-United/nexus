use diesel::{SqliteConnection, connection::SimpleConnection};
use tracing::info;

use crate::expr::ExpressionError;

pub(super) fn setup_schema(conn: &mut SqliteConnection) -> Result<(), ExpressionError> {
    info!("Setting up database schema");
    conn.batch_execute(
        "
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS people (
            id TEXT PRIMARY KEY,
            given_name TEXT NOT NULL,
            surname TEXT NOT NULL,
            suffix TEXT,
            prefix TEXT
        );

        CREATE TABLE IF NOT EXISTS sources (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS polls (
            id TEXT PRIMARY KEY,
            source_id TEXT NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
            published_timestamp TEXT NOT NULL,
            UNIQUE (source_id, published_timestamp)
        );

        CREATE TABLE IF NOT EXISTS questions (
            id TEXT PRIMARY KEY,
            text TEXT NOT NULL,
            keywords TEXT NOT NULL,
            poll_id TEXT NOT NULL REFERENCES polls(id) ON DELETE CASCADE,
            UNIQUE (poll_id, text)
        );

        CREATE TABLE IF NOT EXISTS response_units (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS demographics (
            id TEXT PRIMARY KEY,
            key TEXT NOT NULL UNIQUE,
            demographic_type TEXT NOT NULL,
            label TEXT,
            lower_bound INTEGER,
            upper_bound INTEGER,
            registered BOOLEAN
        );

        CREATE TABLE IF NOT EXISTS responses (
            id TEXT PRIMARY KEY,
            question_id TEXT NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
            demographic_id TEXT NOT NULL REFERENCES demographics(id) ON DELETE CASCADE,
            unit_id TEXT NOT NULL REFERENCES response_units(id) ON DELETE CASCADE,
            answer TEXT NOT NULL,
            value INTEGER NOT NULL,
            UNIQUE (question_id, demographic_id, unit_id, answer, value)
        );

        CREATE INDEX IF NOT EXISTS responses_question_id_idx ON responses (question_id);
        CREATE INDEX IF NOT EXISTS questions_poll_id_idx ON questions (poll_id);
        ",
    )?;

    Ok(())
}
