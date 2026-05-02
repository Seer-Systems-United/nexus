//! # Nexus server entry point
//!
//! This binary is the main entry point for the Nexus Public Polling Federation
//! System. It handles two execution modes:
//!
//! 1. **Server mode** (default): Starts the Axum HTTP server with the React
//!    frontend, API endpoints, OpenAPI docs, and health checks.
//! 2. **CLI mode** (`enrich-topics`): Runs topic enrichment via the
//!    `topics::enrichment::run_cli` interface for batch processing.
//!
//! ## Server configuration
//!
//! - Listens on `127.0.0.1:8080` by default.
//! - Serves the React frontend from `frontend/dist`.
//! - Exposes OpenAPI documentation at `/docs` via Scalar.
//! - Uses `RUST_LOG` or `nexus=info,tower_http=info` for tracing.

use axum::{Router, routing::get};
use nexus::{AppState, api, health, topics};
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt};
use utoipa_scalar::{Scalar, Servable};

/// Entry point for the Nexus server or CLI enrichment tool.
///
/// # Execution modes
///
/// - **Default (no args)**: Starts the Axum HTTP server with API, docs,
///   and hosted React frontend.
/// - **`enrich-topics` arg**: Runs the topic enrichment CLI and exits.
///
/// # Panics
///
/// Panics if:
///
/// - Tracing initialization fails.
/// - TCP listener cannot bind to `127.0.0.1:8080`.
/// - Database or state initialization fails.
#[tokio::main]
async fn main() {
    init_tracing();

    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.first().map(String::as_str) == Some("enrich-topics") {
        args.remove(0);
        if let Err(error) = topics::enrichment::run_cli(args).await {
            eprintln!("enrich-topics failed: {error}");
            std::process::exit(1);
        }
        return;
    }

    let state = AppState::from_env();
    let (api_router, openapi) = api::get_openapi().split_for_parts();
    let scalar_router: Router<AppState> = Scalar::with_url("/docs", openapi).into();

    let app = Router::new()
        .route("/health", get(health::health))
        .nest("/api", api_router)
        .merge(scalar_router)
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .fallback_service(ServeFile::new("frontend/dist/index.html"))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind TCP listener");

    info!(address = %listener.local_addr().unwrap(), "listening");

    axum::serve(listener, app).await.expect("server failed");
}

/// Initializes the `tracing` subscriber with environment-based filtering.
///
/// # Configuration
///
/// - Reads `RUST_LOG` environment variable if set.
/// - Falls back to `nexus=info,tower_http=info` if `RUST_LOG` is not set.
///
/// # Behavior
///
/// - Disables target printing in log output for cleaner logs.
/// - Uses compact formatting for human-readable output.
fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("nexus=info,tower_http=info"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();
}
