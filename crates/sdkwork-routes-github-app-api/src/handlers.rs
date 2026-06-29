use axum::extract::{Query, State};
use axum::response::{Redirect, Response};
use axum::Json;
use http::StatusCode;
use sdkwork_github_integration_service::ports::GitHubSyncStore;
use sdkwork_routes_github_common::{
    finish_api_json, item_data, list_page_data, map_service_error, ApiProblem,
};
use sdkwork_utils_rust::string::is_blank;
use sdkwork_web_core::WebRequestContext;

use crate::dto::{
    CatalogBootstrapResponse, IntegrationStatusResponse, LinkIntegrationRequest,
    OAuthBeginResponse, OAuthCallbackQuery, PageQuery, SyncResponse,
};
use crate::state::GitHubAppState;

fn resolve_scope(
    app_ctx: &WebRequestContext,
    query: &PageQuery,
) -> Result<(String, String), ApiProblem> {
    let principal = app_ctx
        .principal
        .as_ref()
        .ok_or_else(|| ApiProblem::unauthorized("authenticated principal is required"))?;
    let tenant_id = query
        .tenant_id
        .clone()
        .filter(|value| !is_blank(Some(value.as_str())))
        .unwrap_or_else(|| principal.tenancy.tenant_id.clone());
    let organization_id = query
        .organization_id
        .clone()
        .filter(|value| !is_blank(Some(value.as_str())))
        .or_else(|| principal.tenancy.organization_id.clone())
        .ok_or_else(|| ApiProblem::bad_request("organization_id is required"))?;
    Ok((tenant_id, organization_id))
}

pub async fn list_repositories<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> Response {
    let result = (|| async {
        let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
        let page = state
            .service
            .list_repositories(&tenant_id, &organization_id, page, page_size)
            .await
            .map_err(map_service_error)?;
        Ok(list_page_data(page.items, page.page, page.page_size, page.total))
    })()
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn sync_repositories<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> Response {
    let result = (|| async {
        let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
        let sync = state
            .service
            .sync_repositories(&tenant_id, &organization_id)
            .await
            .map_err(map_service_error)?;
        Ok(item_data(SyncResponse::from(sync)))
    })()
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn list_issues<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> Response {
    let result = (|| async {
        let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
        let repository_id = query.repository_id.as_deref();
        let page = state
            .service
            .list_issues(
                &tenant_id,
                &organization_id,
                repository_id,
                page,
                page_size,
            )
            .await
            .map_err(map_service_error)?;
        Ok(list_page_data(page.items, page.page, page.page_size, page.total))
    })()
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn sync_issues<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> Response {
    let result = (|| async {
        let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
        let repository_id = query.repository_id.as_deref();
        let sync = state
            .service
            .sync_issues(&tenant_id, &organization_id, repository_id)
            .await
            .map_err(map_service_error)?;
        Ok(item_data(SyncResponse::from(sync)))
    })()
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn list_plans<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> Response {
    let result = (|| async {
        let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
        let page = state
            .service
            .list_plans(&tenant_id, &organization_id, page, page_size)
            .await
            .map_err(map_service_error)?;
        Ok(list_page_data(page.items, page.page, page.page_size, page.total))
    })()
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn get_integration_status<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> Response {
    let result = (|| async {
        let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
        let status = state
            .service
            .get_integration_status(&tenant_id, &organization_id)
            .await
            .map_err(map_service_error)?;
        Ok(item_data(IntegrationStatusResponse::from(status)))
    })()
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn link_integration<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
    Json(body): Json<LinkIntegrationRequest>,
) -> Response {
    let result = (|| async {
        let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
        let status = state
            .service
            .link_integration(&tenant_id, &organization_id, body.into())
            .await
            .map_err(map_service_error)?;
        Ok(item_data(IntegrationStatusResponse::from(status)))
    })()
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn unlink_integration<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> Response {
    let result = (|| async {
        let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
        let status = state
            .service
            .unlink_integration(&tenant_id, &organization_id)
            .await
            .map_err(map_service_error)?;
        Ok(item_data(IntegrationStatusResponse::from(status)))
    })()
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn bootstrap_notable_catalog<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> Response {
    let result = (|| async {
        let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
        let bootstrap = state
            .service
            .bootstrap_notable_catalog(&tenant_id, &organization_id)
            .await
            .map_err(map_service_error)?;
        Ok(item_data(CatalogBootstrapResponse::from(bootstrap)))
    })()
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn begin_oauth_integration<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> Response {
    let result = (|| async {
        let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
        let oauth = state
            .service
            .begin_oauth_integration(&tenant_id, &organization_id)
            .await
            .map_err(map_service_error)?;
        Ok(item_data(OAuthBeginResponse::from(oauth)))
    })()
    .await;
    finish_api_json(&app_ctx, result)
}

pub async fn oauth_callback<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    Query(query): Query<OAuthCallbackQuery>,
) -> Result<Redirect, (StatusCode, String)> {
    let success_redirect = std::env::var("SDKWORK_GITHUB_OAUTH_SUCCESS_REDIRECT_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:5175/integration".to_string());
    match state
        .service
        .complete_oauth_integration(&query.state, &query.code)
        .await
    {
        Ok(_) => Ok(Redirect::temporary(&format!("{success_redirect}?linked=1"))),
        Err(error) => {
            let error_message = error.to_string();
            let message = urlencoding::encode(&error_message);
            Ok(Redirect::temporary(&format!(
                "{success_redirect}?linked=0&error={message}"
            )))
        }
    }
}
