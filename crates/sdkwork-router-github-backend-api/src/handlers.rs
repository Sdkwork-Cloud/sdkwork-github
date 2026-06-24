use axum::extract::{Query, State};
use axum::Json;
use http::StatusCode;
use sdkwork_github_integration_service::error::ServiceError;
use sdkwork_github_integration_service::ports::GitHubSyncStore;

use crate::dto::{AdminIntegrationPageResponse, AdminSyncRequest, CatalogBootstrapResponse, PageQuery, SyncResponse};
use crate::state::GitHubBackendState;

type ApiResult<T> = Result<T, (StatusCode, String)>;

fn map_service_error(error: ServiceError) -> (StatusCode, String) {
    match error {
        ServiceError::Validation(message) => (StatusCode::BAD_REQUEST, message),
        ServiceError::Configuration(message) => (StatusCode::SERVICE_UNAVAILABLE, message),
        ServiceError::Integration(message) => (StatusCode::BAD_GATEWAY, message),
        ServiceError::Repository(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
    }
}

pub async fn list_integrations<S: GitHubSyncStore>(
    State(state): State<GitHubBackendState<S>>,
    Query(query): Query<PageQuery>,
) -> ApiResult<Json<AdminIntegrationPageResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
    let result = state
        .service
        .list_admin_integrations(page, page_size)
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
}

pub async fn sync_integration_repositories<S: GitHubSyncStore>(
    State(state): State<GitHubBackendState<S>>,
    Json(body): Json<AdminSyncRequest>,
) -> ApiResult<Json<SyncResponse>> {
    let result = state
        .service
        .sync_repositories(&body.tenant_id, &body.organization_id)
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
}

pub async fn sync_notable_catalog<S: GitHubSyncStore>(
    State(state): State<GitHubBackendState<S>>,
    Json(body): Json<AdminSyncRequest>,
) -> ApiResult<Json<CatalogBootstrapResponse>> {
    let result = state
        .service
        .bootstrap_notable_catalog(&body.tenant_id, &body.organization_id)
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
}
