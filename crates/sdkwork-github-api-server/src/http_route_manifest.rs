use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

use sdkwork_router_github_app_api::app_route_manifest;
use sdkwork_router_github_backend_api::backend_route_manifest;

const fn concat_route_manifests<const A: usize, const B: usize>(
    left: [HttpRoute; A],
    right: [HttpRoute; B],
) -> [HttpRoute; A + B] {
    let mut merged = [left[0]; A + B];
    let mut index = 0;
    while index < A {
        merged[index] = left[index];
        index += 1;
    }
    let mut right_index = 0;
    while right_index < B {
        merged[A + right_index] = right[right_index];
        right_index += 1;
    }
    merged
}

const APP_ROUTES: [HttpRoute; 10] = *app_route_manifest().routes();
const BACKEND_ROUTES: [HttpRoute; 2] = *backend_route_manifest().routes();
const GITHUB_HTTP_ROUTES: [HttpRoute; 12] =
    concat_route_manifests(APP_ROUTES, BACKEND_ROUTES);

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
