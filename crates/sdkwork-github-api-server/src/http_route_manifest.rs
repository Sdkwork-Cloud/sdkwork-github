use sdkwork_web_core::HttpRouteManifest;

use sdkwork_router_github_app_api::app_route_manifest;

pub fn github_route_manifest() -> HttpRouteManifest {
    app_route_manifest()
}

pub fn github_public_path_prefixes() -> Vec<String> {
    vec!["/health".to_string(), "/healthz".to_string()]
}
