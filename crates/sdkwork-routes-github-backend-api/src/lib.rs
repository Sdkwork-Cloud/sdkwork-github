pub mod dto;
pub mod handlers;
pub mod http_route_manifest;
pub mod paths;
pub mod routes;
pub mod state;

pub use http_route_manifest::{backend_route_manifest, BACKEND_HTTP_ROUTES};
pub use routes::build_router;

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    backend_route_manifest()
}

pub fn gateway_mount<S>(service: sdkwork_github_integration_service::GitHubIntegrationService<S>) -> axum::Router
where
    S: sdkwork_github_integration_service::ports::GitHubSyncStore + Clone + Send + Sync + 'static,
{
    build_router(service)
}
