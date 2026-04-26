pub mod api;
pub mod database;
pub mod health;
pub mod state;
pub mod utils;

pub use state::AppState;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use utoipa_scalar::{Scalar, Servable};

#[tokio::main]
async fn main() {
    let state = AppState::from_env();
    let (api_router, openapi) = api::get_openapi().split_for_parts();
    let scalar_router: Router<AppState> = Scalar::with_url("/docs", openapi).into();

    let app = Router::new()
        .route("/health", get(health::health))
        .nest("/api", api_router)
        .merge(scalar_router)
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .fallback_service(ServeFile::new("frontend/dist/index.html"))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind TCP listener");

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server failed");
}
