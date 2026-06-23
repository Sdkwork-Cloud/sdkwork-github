use axum::extract::{Query, State};
use axum::Json;
use http::StatusCode;
use sdkwork_github_integration_service::ports::GitHubStore;
use sdkwork_utils_rust::string::is_blank;
use sdkwork_web_core::WebRequestContext;

use crate::dto::{IssuePageResponse, PageQuery, PlanPageResponse, RepositoryPageResponse};
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

pub async fn list_repositories<S: GitHubStore>(
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
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    Ok(Json(result.into()))
}

pub async fn list_issues<S: GitHubStore>(
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
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    Ok(Json(result.into()))
}

pub async fn list_plans<S: GitHubStore>(
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
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    Ok(Json(result.into()))
}
