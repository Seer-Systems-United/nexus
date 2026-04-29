postgres-root := ".postgres-test"
postgres-data := ".postgres-test/data"
postgres-log := ".postgres-test/postgres.log"
postgres-port := "55432"
postgres-user := "nexus"
postgres-db := "nexus_test"
database-url := "postgres://nexus@127.0.0.1:55432/nexus_test"

run:
    nix-shell --run 'just _run'

run-test-db:
    nix-shell --run 'just _run-test-db'

test:
    nix-shell --run 'just _test'

postgres-setup:
    nix-shell --run 'just _postgres-setup'

postgres-start:
    nix-shell --run 'just _postgres-start'

postgres-stop:
    nix-shell --run 'just _postgres-stop'

postgres-reset:
    nix-shell --run 'just _postgres-reset'

postgres-url:
    @echo {{database-url}}

[private]
_run:
    cd frontend && pnpm install --frozen-lockfile
    cd frontend && pnpm build
    cargo run

[private]
_run-test-db: _postgres-setup
    cd frontend && pnpm install --frozen-lockfile
    cd frontend && pnpm build
    DATABASE_URL='{{database-url}}' cargo run

[private]
_test: _postgres-setup
    DATABASE_URL='{{database-url}}' cargo test

[private]
_postgres-setup: _postgres-start _postgres-create-db _postgres-schema

[private]
_postgres-start:
    #!/usr/bin/env bash
    set -euo pipefail

    mkdir -p '{{postgres-root}}'

    if [ ! -d '{{postgres-data}}' ]; then
      initdb \
        --auth=trust \
        --encoding=UTF8 \
        --no-locale \
        --username='{{postgres-user}}' \
        -D '{{postgres-data}}' \
        >/dev/null
      echo 'created postgres data directory at {{postgres-data}}'
    fi

    if pg_ctl -D '{{postgres-data}}' status >/dev/null 2>&1; then
      echo 'postgres already running on port {{postgres-port}}'
    else
      socket_dir="$(pwd)/{{postgres-root}}"
      pg_ctl \
        -D '{{postgres-data}}' \
        -l '{{postgres-log}}' \
        -o "-h 127.0.0.1 -p {{postgres-port}} -c unix_socket_directories=$socket_dir" \
        start
    fi

    pg_isready \
      -h 127.0.0.1 \
      -p '{{postgres-port}}' \
      -U '{{postgres-user}}' \
      >/dev/null

[private]
_postgres-create-db:
    #!/usr/bin/env bash
    set -euo pipefail

    db_exists=$(
      psql \
        -h 127.0.0.1 \
        -p '{{postgres-port}}' \
        -U '{{postgres-user}}' \
        -d postgres \
        -tAc "SELECT 1 FROM pg_database WHERE datname = '{{postgres-db}}'"
    )

    if [ "$db_exists" = "1" ]; then
      echo 'database {{postgres-db}} already exists'
    else
      createdb \
        -h 127.0.0.1 \
        -p '{{postgres-port}}' \
        -U '{{postgres-user}}' \
        '{{postgres-db}}'
      echo 'created database {{postgres-db}}'
    fi

[private]
_postgres-schema:
    #!/usr/bin/env bash
    set -euo pipefail

    psql '{{database-url}}' -v ON_ERROR_STOP=1 <<'SQL'
    CREATE TABLE IF NOT EXISTS users (
      id UUID PRIMARY KEY,
      name TEXT NOT NULL,
      email TEXT,
      account_number TEXT,
      created_at TIMESTAMP NOT NULL
    );

    ALTER TABLE users
      ALTER COLUMN email DROP NOT NULL;

    ALTER TABLE users
      ADD COLUMN IF NOT EXISTS account_number TEXT;

    ALTER TABLE users
      DROP CONSTRAINT IF EXISTS users_email_key;

    CREATE UNIQUE INDEX IF NOT EXISTS users_email_unique_idx
      ON users(email)
      WHERE email IS NOT NULL;

    CREATE UNIQUE INDEX IF NOT EXISTS users_account_number_unique_idx
      ON users(account_number)
      WHERE account_number IS NOT NULL;

    CREATE TABLE IF NOT EXISTS login (
      id UUID PRIMARY KEY,
      user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      password_hash TEXT NOT NULL,
      created_at TIMESTAMP NOT NULL
    );

    CREATE INDEX IF NOT EXISTS login_user_id_created_at_idx
      ON login(user_id, created_at DESC);
    SQL

[private]
_postgres-reset: _postgres-start
    dropdb --if-exists -h 127.0.0.1 -p '{{postgres-port}}' -U '{{postgres-user}}' '{{postgres-db}}'
    just _postgres-create-db
    just _postgres-schema

[private]
_postgres-stop:
    #!/usr/bin/env bash
    set -euo pipefail

    if [ -d '{{postgres-data}}' ] && pg_ctl -D '{{postgres-data}}' status >/dev/null 2>&1; then
      pg_ctl -D '{{postgres-data}}' stop
    else
      echo 'postgres is not running'
    fi
