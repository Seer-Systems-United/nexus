-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

DROP TABLE IF EXISTS responses;
DROP TABLE IF EXISTS demographics;
DROP TABLE IF EXISTS response_units;
DROP TABLE IF EXISTS questions;
DROP TABLE IF EXISTS polls;
DROP TABLE IF EXISTS sources;
DROP TABLE IF EXISTS people;

DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();
