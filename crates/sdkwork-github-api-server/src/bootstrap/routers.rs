use axum::Router;
use tower_http::cors::CorsLayer;

use crate::bootstrap::auth::build_protected_router;
use crate::bootstrap::database::build_github_service;
use crate::health;

pub async fn build_router() -> Result<Router, Box<dyn std::error::Error + Send + Sync>> {
    let service = build_github_service()
        .await
        .map_err(|error| -> Box<dyn std::error::Error + Send + Sync> { error.into() })?;

    let iam_router = sdkwork_router_iam_app_api::build_sdkwork_appbase_app_api_router()
        .await
        .map_err(|error| -> Box<dyn std::error::Error + Send + Sync> { error.into() })?;

    let github_app_router = sdkwork_router_github_app_api::routes::build_router(service.clone());
    let github_backend_router =
        sdkwork_router_github_backend_api::build_router(service);

    let protected = Router::new()
        .merge(github_app_router)
        .merge(github_backend_router);

    let app = Router::new()
        .merge(iam_router)
        .merge(build_protected_router(protected).await)
        .route("/health", axum::routing::get(health::health_check))
        .route("/healthz", axum::routing::get(health::health_check))
        .layer(CorsLayer::permissive());

    Ok(app)
}
