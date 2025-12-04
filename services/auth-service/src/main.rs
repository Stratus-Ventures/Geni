use dotenv::dotenv;
use auth_service::infra::setup::{init_app_state, init_tracing};
use auth_service::adapters::http::router::api_router;
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load environment variables
    dotenv().ok();

    // 2. Initialize Tracing
    init_tracing();

    // 3. Initialize Application State (DB connections, Service Wiring)
    let app_state = init_app_state().await?;
    let port = app_state.config.port;

    // 4. Build Router
    let app = api_router(app_state);

    // 5. Bind and Serve
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Auth Service listening on {}", addr);

    // Use into_make_service_with_connect_info to provide SocketAddr for rate limiting
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await?;

    Ok(())
}
