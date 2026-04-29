# nexus
Public Polling Federation System

## Development

Run the Axum API:

```sh
cargo run
```

Build the frontend and run the Axum server that hosts it:

```sh
just run
```

Set `RUST_LOG` to tune API logs. The default is `nexus=info,tower_http=info`.

```sh
RUST_LOG=nexus=debug,tower_http=debug cargo run
```

Run the React dev server:

```sh
cd frontend
pnpm install
pnpm dev
```

Vite proxies `/health`, `/api`, and `/docs` to Axum at `http://127.0.0.1:8080`.

Password signup creates a local webhook account with a generated 16-digit
account number. Google OpenID accounts are still keyed by verified email.

Polling source ingestion is exposed through Axum:

```text
GET /api/v1/sources/
GET /api/v1/sources/emerson
GET /api/v1/sources/gallup
GET /api/v1/sources/yougov
```

Source data endpoints accept `scope` and `count` query parameters:

```text
?scope=latest
?scope=last_n_entries&count=5
?scope=last_days&count=30
?scope=last_weeks&count=4
?scope=last_months&count=6
?scope=last_years&count=1
```

Source refreshes use `data/<source-name>/<scope>/` cache files and emit
structured tracing events for cache hits, stale-cache fallbacks, downloads,
parsing, and skipped source assets.

Google OpenID login reads these environment variables:

```sh
GOOGLE_CLIENT_ID=...
GOOGLE_CLIENT_SECRET=...
GOOGLE_REDIRECT_URL=http://127.0.0.1:8080/api/v1/auth/google/callback
```

`GOOGLE_REDIRECT_URL` is optional for local development if the Google OAuth
client is configured with the default callback above.

## Hosting React From Axum

Build the frontend before starting the Rust server:

```sh
cd frontend
pnpm build
cd ..
cargo run
```

Axum serves the built React app from `frontend/dist` and falls back to
`index.html` for client-side routes.
