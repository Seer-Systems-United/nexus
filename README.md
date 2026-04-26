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

Run the React dev server:

```sh
cd frontend
pnpm install
pnpm dev
```

Vite proxies `/health` to Axum at `http://127.0.0.1:3000`.

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
