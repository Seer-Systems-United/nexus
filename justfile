set dotenv-load := true

database-url := env_var_or_default("DATABASE_URL", "postgres://nexus@127.0.0.1:55432/nexus_test")

default:
    just --list

# Remove application data while keeping the schema and Diesel migration history.
clear-db:
    psql "{{database-url}}" -v ON_ERROR_STOP=1 -c "DO \$\$ DECLARE table_names text; BEGIN SELECT string_agg(format('%I', table_name), ', ' ORDER BY array_position(ARRAY['responses','demographics','response_units','questions','polls','sources','people'], table_name)) INTO table_names FROM information_schema.tables WHERE table_schema = 'public' AND table_name = ANY (ARRAY['responses','demographics','response_units','questions','polls','sources','people']); IF table_names IS NOT NULL THEN EXECUTE 'TRUNCATE TABLE ' || table_names || ' RESTART IDENTITY CASCADE'; END IF; END \$\$;"

db-clear: clear-db
