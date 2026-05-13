DROP TABLE IF EXISTS responses;
DROP TABLE IF EXISTS demographics;
DROP TABLE IF EXISTS response_units;

DROP INDEX IF EXISTS questions_poll_id_text_idx;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'questions_poll_id_fkey'
          AND conrelid = 'questions'::regclass
    ) THEN
        ALTER TABLE questions DROP CONSTRAINT questions_poll_id_fkey;
    END IF;
END
$$;

ALTER TABLE questions DROP COLUMN IF EXISTS poll_id;

DROP TABLE IF EXISTS polls;
DROP TABLE IF EXISTS sources;
