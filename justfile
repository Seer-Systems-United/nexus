set dotenv-load := true

database-url := env_var_or_default("DATABASE_URL", "postgres://nexus@127.0.0.1:55432/nexus_test")

default:
    just --list

# Remove application data while keeping the schema and Diesel migration history.
clear-db:
    psql "{{database-url}}" -v ON_ERROR_STOP=1 -c 'TRUNCATE TABLE questions, people RESTART IDENTITY CASCADE;'

db-clear: clear-db
