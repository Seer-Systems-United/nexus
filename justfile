run:
    cd frontend && pnpm install --frozen-lockfile
    cd frontend && pnpm build
    cargo run
