use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &BACKEND_HTTP_ROUTES;

pub const BACKEND_HTTP_ROUTES: [HttpRoute; 3] = [
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/github/integrations",
        "github",
        "integrations.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/backend/v3/api/github/integrations/sync",
        "github",
        "integrations.repositories.sync",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/backend/v3/api/github/catalog/sync",
        "github",
        "catalog.sync",
    ),
];

pub fn backend_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
