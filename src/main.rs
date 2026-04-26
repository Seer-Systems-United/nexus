pub mod health;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health::health))
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .fallback_service(ServeFile::new("frontend/dist/index.html"));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind TCP listener");

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server failed");
}
