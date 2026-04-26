run:
    nix-shell --run 'just _run'

[private]
_run:
    cd frontend && pnpm install --frozen-lockfile
    cd frontend && pnpm build
    cargo run
