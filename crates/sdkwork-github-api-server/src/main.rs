use sdkwork_github_api_server::build_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let bind_address = std::env::var("SDKWORK_GITHUB_APPLICATION_PUBLIC_INGRESS_BIND")
        .unwrap_or_else(|_| "127.0.0.1:4100".to_string());
    let app = build_router()
        .await
        .expect("github api-server bootstrap failed");
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("bind github api-server listener failed");
    tracing::info!("sdkwork-github-api-server listening on {bind_address}");
    axum::serve(listener, app)
        .await
        .expect("serve github api-server failed");
}
