use axum::http::StatusCode;
use axum::Json;
use sdkwork_database_sqlx::DatabasePool;
use serde_json::json;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "sdkwork-github-api-server"
    }))
}

pub async fn ready_check(pool: DatabasePool) -> (StatusCode, Json<serde_json::Value>) {
    let database = match ping_database(&pool).await {
        Ok(()) => "ok",
        Err(message) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "status": "degraded",
                    "service": "sdkwork-github-api-server",
                    "database": message,
                })),
            );
        }
    };

    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "sdkwork-github-api-server",
            "database": database,
        })),
    )
}

pub async fn metrics_snapshot() -> Json<serde_json::Value> {
    let provider = sdkwork_github_integration_provider_github::provider_metrics_snapshot();
    Json(json!({
        "service": "sdkwork-github-api-server",
        "github_provider": provider,
    }))
}

async fn ping_database(pool: &DatabasePool) -> Result<(), String> {
    match pool {
        DatabasePool::Sqlite(sqlite, _) => sqlx::query("SELECT 1")
            .execute(sqlite)
            .await
            .map(|_| ())
            .map_err(|error| format!("sqlite ping failed: {error}")),
        DatabasePool::Postgres(postgres, _) => sqlx::query("SELECT 1")
            .execute(postgres)
            .await
            .map(|_| ())
            .map_err(|error| format!("postgres ping failed: {error}")),
    }
}
