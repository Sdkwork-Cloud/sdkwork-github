use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/github/repositories",
        "github",
        "repositories.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/github/issues",
        "github",
        "issues.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/github/plans",
        "github",
        "plans.list",
    ),
];

pub fn app_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
