//! Gateway assembly for sdkwork-github.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.

mod bootstrap;
mod generated;

pub use bootstrap::{assemble_application_business_router, ApplicationAssembly};
pub use sdkwork_routes_github_app_api::{app_route_manifest, APP_HTTP_ROUTES};
pub use sdkwork_routes_github_backend_api::{backend_route_manifest, BACKEND_HTTP_ROUTES};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}

pub use generated::ROUTE_CRATE_PACKAGES;
