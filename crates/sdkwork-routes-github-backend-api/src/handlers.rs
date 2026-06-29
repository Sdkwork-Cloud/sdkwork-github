use axum::extract::{Query, State};
use axum::response::Response;
use axum::Json;
use sdkwork_github_integration_service::ports::GitHubSyncStore;
use sdkwork_routes_github_common::{
    finish_api_json, item_data, list_page_data, map_service_error,
};
use sdkwork_web_core::WebRequestContext;

use crate::dto::{AdminSyncRequest, CatalogBootstrapResponse, PageQuery, SyncResponse};
use crate::state::GitHubBackendState;

pub async fn list_integrations<S: GitHubSyncStore>(
    State(state): State<GitHubBackendState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> Response {
    let result = async {
        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
        let page = state
            .service
            .list_admin_integrations(page, page_size)
            .await
            .map_err(map_service_error)?;
        Ok(list_page_data(page.items, page.page, page.page_size, page.total))
    }
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn sync_integration_repositories<S: GitHubSyncStore>(
    State(state): State<GitHubBackendState<S>>,
    app_ctx: WebRequestContext,
    Json(body): Json<AdminSyncRequest>,
) -> Response {
    let result = async {
        let sync = state
            .service
            .sync_repositories(&body.tenant_id, &body.organization_id)
            .await
            .map_err(map_service_error)?;
        Ok(item_data(SyncResponse::from(sync)))
    }
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn sync_notable_catalog<S: GitHubSyncStore>(
    State(state): State<GitHubBackendState<S>>,
    app_ctx: WebRequestContext,
    Json(body): Json<AdminSyncRequest>,
) -> Response {
    let result = async {
        let bootstrap = state
            .service
            .bootstrap_notable_catalog(&body.tenant_id, &body.organization_id)
            .await
            .map_err(map_service_error)?;
        Ok(item_data(CatalogBootstrapResponse::from(bootstrap)))
    }
    .await;
    finish_api_json(&app_ctx, result)
}
