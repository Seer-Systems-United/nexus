use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_axum::router::OpenApiRouter;

pub mod auth;
pub mod dashboard;
mod db;
pub mod error;
pub mod sources;
pub mod topics;

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    servers((url = "/api")),
    tags(
        (name = "Auth", description = "Authentication and account creation"),
        (name = "Dashboard", description = "Authenticated dashboard access"),
        (name = "Sources", description = "Polling source ingestion"),
        (name = "Topics", description = "Canonical polling question topics"),
    )
)]
struct NexusApiDoc;

pub fn get_openapi() -> OpenApiRouter<crate::AppState> {
    OpenApiRouter::with_openapi(NexusApiDoc::openapi())
        .nest("/v1/auth", auth::get_openapi())
        .nest("/v1/dashboard", dashboard::get_openapi())
        .nest("/v1/sources", sources::get_openapi())
        .nest("/v1/topics", topics::get_openapi())
}

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .get_or_insert_with(utoipa::openapi::Components::new)
            .add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
    }
}
