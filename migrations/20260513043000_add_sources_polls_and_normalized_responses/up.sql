CREATE TABLE IF NOT EXISTS sources (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

INSERT INTO sources (id, name)
SELECT '00000000-0000-0000-0000-000000000001', 'Legacy'
WHERE EXISTS (
    SELECT 1 FROM questions
)
AND NOT EXISTS (
    SELECT 1 FROM sources WHERE name = 'Legacy'
);

CREATE TABLE IF NOT EXISTS polls (
    id UUID PRIMARY KEY,
    source_id UUID NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
    published_timestamp TIMESTAMP NOT NULL,
    UNIQUE (source_id, published_timestamp)
);

INSERT INTO polls (id, source_id, published_timestamp)
SELECT '00000000-0000-0000-0000-000000000002', sources.id, TIMESTAMP '1970-01-01 00:00:00'
FROM sources
WHERE sources.name = 'Legacy'
  AND EXISTS (
      SELECT 1 FROM questions
  )
ON CONFLICT (source_id, published_timestamp) DO NOTHING;

ALTER TABLE questions ADD COLUMN IF NOT EXISTS poll_id UUID;

UPDATE questions
SET poll_id = (
    SELECT polls.id
    FROM polls
    INNER JOIN sources ON sources.id = polls.source_id
    WHERE sources.name = 'Legacy'
      AND polls.published_timestamp = TIMESTAMP '1970-01-01 00:00:00'
    LIMIT 1
)
WHERE poll_id IS NULL;

ALTER TABLE questions ALTER COLUMN poll_id SET NOT NULL;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'questions_poll_id_fkey'
          AND conrelid = 'questions'::regclass
    ) THEN
        ALTER TABLE questions
            ADD CONSTRAINT questions_poll_id_fkey
            FOREIGN KEY (poll_id) REFERENCES polls(id) ON DELETE CASCADE;
    END IF;
END
$$;

CREATE UNIQUE INDEX IF NOT EXISTS questions_poll_id_text_idx ON questions (poll_id, text);

CREATE TABLE IF NOT EXISTS response_units (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS demographics (
    id UUID PRIMARY KEY,
    key VARCHAR NOT NULL UNIQUE,
    demographic_type VARCHAR NOT NULL,
    label VARCHAR,
    lower_bound INTEGER,
    upper_bound INTEGER,
    registered BOOLEAN
);

DO $$
BEGIN
    IF to_regclass('public.responses') IS NOT NULL
       AND (
           NOT EXISTS (
               SELECT 1 FROM information_schema.columns
               WHERE table_schema = 'public'
                 AND table_name = 'responses'
                 AND column_name = 'question_id'
           )
           OR NOT EXISTS (
               SELECT 1 FROM information_schema.columns
               WHERE table_schema = 'public'
                 AND table_name = 'responses'
                 AND column_name = 'demographic_id'
           )
           OR NOT EXISTS (
               SELECT 1 FROM information_schema.columns
               WHERE table_schema = 'public'
                 AND table_name = 'responses'
                 AND column_name = 'unit_id'
           )
           OR NOT EXISTS (
               SELECT 1 FROM information_schema.columns
               WHERE table_schema = 'public'
                 AND table_name = 'responses'
                 AND column_name = 'answer'
           )
           OR NOT EXISTS (
               SELECT 1 FROM information_schema.columns
               WHERE table_schema = 'public'
                 AND table_name = 'responses'
                 AND column_name = 'value'
           )
       ) THEN
        DROP TABLE responses;
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS responses (
    id UUID PRIMARY KEY,
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    demographic_id UUID NOT NULL REFERENCES demographics(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES response_units(id) ON DELETE CASCADE,
    answer VARCHAR NOT NULL,
    value INTEGER NOT NULL,
    UNIQUE (question_id, demographic_id, unit_id, answer, value)
);

CREATE INDEX IF NOT EXISTS responses_question_id_idx ON responses (question_id);
