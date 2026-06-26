use axum::Router;
use axum::http::{HeaderValue, Method};
use std::sync::Arc;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::bootstrap::auth::build_protected_router;
use crate::bootstrap::database::build_github_bootstrap;
use crate::readiness::GithubDatabaseReadinessCheck;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};

fn build_cors_layer() -> CorsLayer {
    let allowed_origins = std::env::var("SDKWORK_GITHUB_CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://127.0.0.1:5175,http://localhost:5175".to_string());
    let origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .filter_map(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                HeaderValue::from_str(trimmed).ok()
            }
        })
        .collect();

    let mut layer = CorsLayer::new()
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::any())
        .allow_credentials(true);

    layer = if origins.is_empty() {
        layer.allow_origin(AllowOrigin::mirror_request())
    } else {
        layer.allow_origin(AllowOrigin::list(origins))
    };

    layer
}

pub async fn build_router() -> Result<Router, Box<dyn std::error::Error + Send + Sync>> {
    let bootstrap = build_github_bootstrap()
        .await
        .map_err(|error| -> Box<dyn std::error::Error + Send + Sync> { error.into() })?;
    let service = bootstrap.service.clone();
    let pool = bootstrap.pool.clone();

    sdkwork_iam_database_host::bootstrap_iam_database_from_env()
        .await
        .map_err(|error| -> Box<dyn std::error::Error + Send + Sync> { error.into() })?;
    crate::bootstrap::iam_application_bootstrap::ensure_github_tenant_application_bootstrap()
        .await
        .map_err(|error| -> Box<dyn std::error::Error + Send + Sync> { error.into() })?;

    let iam_router = sdkwork_routes_iam_app_api::build_sdkwork_iam_app_api_router()
        .await
        .map_err(|error| -> Box<dyn std::error::Error + Send + Sync> { error.into() })?;

    let github_app_router = sdkwork_routes_github_app_api::routes::build_router(service.clone());
    let github_backend_router =
        sdkwork_routes_github_backend_api::build_router(service);

    let protected = Router::new()
        .merge(github_app_router)
        .merge(github_backend_router);

    let business = Router::new()
        .merge(iam_router)
        .merge(build_protected_router(protected).await)
        .layer(build_cors_layer());

    Ok(service_router(
        business,
        ServiceRouterConfig::default().with_readiness_check(Arc::new(
            GithubDatabaseReadinessCheck::new(pool),
        )),
    ))
}
