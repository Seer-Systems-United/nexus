//! # `nexus`
//!
//! `nexus` is a Public Polling Federation System that aggregates, processes,
//! and serves polling data from multiple sources (Emerson, Gallup, YouGov)
//! with canonical topic classification, user authentication, and a hosted
//! frontend dashboard.
//!
//! This crate provides:
//!
//! - **API module** (`nexus::api`): Axum-based HTTP API with OpenAPI
//!   documentation, authentication (Google OpenID + password), polling
//!   source ingestion endpoints, and canonical topic management.
//! - **Database module** (`nexus::database`): PostgreSQL connection pooling
//!   and CRUD operations for users, passwords, and polling data using Diesel.
//! - **Health module** (`nexus::health`): Simple health check endpoint for
//!   service monitoring.
//! - **Sources module** (`nexus::sources`): Polling source ingestion logic
//!   for Emerson, Gallup, and YouGov with caching and refresh support.
//! - **State module** (`nexus::state`): Shared application state used
//!   throughout the API and background services.
//! - **Topics module** (`nexus::topics`): Canonical polling question topic
//!   classification, NLP enrichment, demographics mapping, and answer
//!   aggregation.
//! - **Utils module** (`nexus::utils`): Shared utilities including JWT
//!   handling and common helper functions.
//!
//! ## Commenting conventions
//!
//! This project uses the following Rust documentation conventions:
//!
//! - `//!` for crate-level or module-level documentation (like this comment).
//! - `///` for public item documentation (structs, enums, traits, functions).
//! - `//` for implementation notes, complex logic clarification, or warnings
//!   inside function bodies.
//!
//! ## Quick start
//!
//! ```rust
//! // Run the server (see main.rs for CLI and server setup)
//! // cargo run
//!
//! // Enrich polling topics via CLI
//! // cargo run -- enrich-topics
//! ```
//!
//! ## Features
//!
//! - Google OpenID Connect authentication with JWT sessions
//! - Password-based signup with webhook account generation
//! - Polling source ingestion with scoped queries (latest, last N days/weeks/months)
//! - Canonical topic classification with NLP and demographic mapping
//! - React frontend hosted directly by the Axum server
//! - OpenAPI 3.0 documentation with Scalar UI at `/docs`

pub mod api;
pub mod database;
pub mod health;
pub mod sources;
pub mod state;
pub mod topics;
pub mod utils;

pub use state::AppState;
