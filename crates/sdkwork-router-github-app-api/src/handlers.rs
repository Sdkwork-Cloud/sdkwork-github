use axum::extract::{Query, State};
use axum::response::Redirect;
use axum::Json;
use http::StatusCode;
use sdkwork_github_integration_service::error::ServiceError;
use sdkwork_github_integration_service::ports::GitHubSyncStore;
use sdkwork_utils_rust::string::is_blank;
use sdkwork_web_core::WebRequestContext;

use crate::dto::{
    IntegrationStatusResponse, IssuePageResponse, LinkIntegrationRequest, OAuthBeginResponse,
    OAuthCallbackQuery, PageQuery, PlanPageResponse, RepositoryPageResponse, SyncResponse,
};
use crate::state::GitHubAppState;

type ApiResult<T> = Result<T, (StatusCode, String)>;

fn resolve_scope(
    app_ctx: &WebRequestContext,
    query: &PageQuery,
) -> Result<(String, String), (StatusCode, String)> {
    let principal = app_ctx.principal.as_ref().ok_or((
        StatusCode::UNAUTHORIZED,
        "authenticated principal is required".to_string(),
    ))?;
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
        .ok_or((
            StatusCode::BAD_REQUEST,
            "organization_id is required".to_string(),
        ))?;
    Ok((tenant_id, organization_id))
}

fn map_service_error(error: ServiceError) -> (StatusCode, String) {
    match error {
        ServiceError::Validation(message) => (StatusCode::BAD_REQUEST, message),
        ServiceError::Configuration(message) => (StatusCode::SERVICE_UNAVAILABLE, message),
        ServiceError::Integration(message) => (StatusCode::BAD_GATEWAY, message),
        ServiceError::Repository(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
    }
}

pub async fn list_repositories<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> ApiResult<Json<RepositoryPageResponse>> {
    let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
    let result = state
        .service
        .list_repositories(&tenant_id, &organization_id, page, page_size)
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
}

pub async fn sync_repositories<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> ApiResult<Json<SyncResponse>> {
    let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
    let result = state
        .service
        .sync_repositories(&tenant_id, &organization_id)
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
}

pub async fn list_issues<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> ApiResult<Json<IssuePageResponse>> {
    let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
    let repository_id = query.repository_id.as_deref();
    let result = state
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
    Ok(Json(result.into()))
}

pub async fn sync_issues<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> ApiResult<Json<SyncResponse>> {
    let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
    let repository_id = query.repository_id.as_deref();
    let result = state
        .service
        .sync_issues(&tenant_id, &organization_id, repository_id)
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
}

pub async fn list_plans<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> ApiResult<Json<PlanPageResponse>> {
    let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
    let result = state
        .service
        .list_plans(&tenant_id, &organization_id, page, page_size)
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
}

pub async fn get_integration_status<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> ApiResult<Json<IntegrationStatusResponse>> {
    let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
    let result = state
        .service
        .get_integration_status(&tenant_id, &organization_id)
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
}

pub async fn link_integration<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
    Json(body): Json<LinkIntegrationRequest>,
) -> ApiResult<Json<IntegrationStatusResponse>> {
    let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
    let result = state
        .service
        .link_integration(&tenant_id, &organization_id, body.into())
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
}

pub async fn unlink_integration<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> ApiResult<Json<IntegrationStatusResponse>> {
    let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
    let result = state
        .service
        .unlink_integration(&tenant_id, &organization_id)
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
}

pub async fn begin_oauth_integration<S: GitHubSyncStore>(
    State(state): State<GitHubAppState<S>>,
    app_ctx: WebRequestContext,
    Query(query): Query<PageQuery>,
) -> ApiResult<Json<OAuthBeginResponse>> {
    let (tenant_id, organization_id) = resolve_scope(&app_ctx, &query)?;
    let result = state
        .service
        .begin_oauth_integration(&tenant_id, &organization_id)
        .await
        .map_err(map_service_error)?;
    Ok(Json(result.into()))
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
