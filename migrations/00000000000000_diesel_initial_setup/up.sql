-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE people (
    id UUID PRIMARY KEY,
    given_name VARCHAR NOT NULL,
    surname VARCHAR NOT NULL,
    suffix VARCHAR,
    prefix VARCHAR
);

CREATE TABLE sources (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE polls (
    id UUID PRIMARY KEY,
    source_id UUID NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
    published_timestamp TIMESTAMP NOT NULL,
    UNIQUE (source_id, published_timestamp)
);

CREATE TABLE questions (
    id UUID PRIMARY KEY,
    text VARCHAR NOT NULL,
    keywords TSVECTOR NOT NULL,
    poll_id UUID NOT NULL REFERENCES polls(id) ON DELETE CASCADE,
    UNIQUE (poll_id, text)
);

CREATE TABLE response_units (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE demographics (
    id UUID PRIMARY KEY,
    key VARCHAR NOT NULL UNIQUE,
    demographic_type VARCHAR NOT NULL,
    label VARCHAR,
    lower_bound INTEGER,
    upper_bound INTEGER,
    registered BOOLEAN
);

CREATE TABLE responses (
    id UUID PRIMARY KEY,
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    demographic_id UUID NOT NULL REFERENCES demographics(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES response_units(id) ON DELETE CASCADE,
    answer VARCHAR NOT NULL,
    value INTEGER NOT NULL,
    UNIQUE (question_id, demographic_id, unit_id, answer, value)
);

CREATE INDEX questions_keywords_idx ON questions USING GIN (keywords);
CREATE INDEX responses_question_id_idx ON responses (question_id);
