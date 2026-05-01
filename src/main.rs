pub mod api;
pub mod database;
pub mod health;
pub mod sources;
pub mod state;
pub mod topics;
pub mod utils;

pub use state::AppState;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt};
use utoipa_scalar::{Scalar, Servable};

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

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("nexus=info,tower_http=info"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();
}
