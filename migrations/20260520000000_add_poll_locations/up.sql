CREATE TABLE IF NOT EXISTS poll_locations (
    id UUID PRIMARY KEY,
    poll_id UUID NOT NULL UNIQUE REFERENCES polls(id) ON DELETE CASCADE,
    location_type VARCHAR NOT NULL,
    country VARCHAR NOT NULL DEFAULT 'US',
    state VARCHAR,
    county VARCHAR,
    label VARCHAR
);

CREATE INDEX IF NOT EXISTS poll_locations_poll_id_idx ON poll_locations (poll_id);

