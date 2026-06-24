use sdkwork_web_core::{HttpRoute, HttpRouteManifest};

use sdkwork_router_github_app_api::APP_HTTP_ROUTES;
use sdkwork_router_github_backend_api::BACKEND_HTTP_ROUTES;

const GITHUB_HTTP_ROUTES: [HttpRoute; 14] = [
    APP_HTTP_ROUTES[0],
    APP_HTTP_ROUTES[1],
    APP_HTTP_ROUTES[2],
    APP_HTTP_ROUTES[3],
    APP_HTTP_ROUTES[4],
    APP_HTTP_ROUTES[5],
    APP_HTTP_ROUTES[6],
    APP_HTTP_ROUTES[7],
    APP_HTTP_ROUTES[8],
    APP_HTTP_ROUTES[9],
    APP_HTTP_ROUTES[10],
    BACKEND_HTTP_ROUTES[0],
    BACKEND_HTTP_ROUTES[1],
    BACKEND_HTTP_ROUTES[2],
];

pub fn github_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(&GITHUB_HTTP_ROUTES)
}

pub fn github_public_path_prefixes() -> Vec<String> {
    vec![
        "/health".to_string(),
        "/healthz".to_string(),
        "/readyz".to_string(),
        "/metrics".to_string(),
    ]
}
