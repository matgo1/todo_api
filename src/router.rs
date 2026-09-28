//! REST API handling
use anyhow::Context;
use axum::{Router, routing};
use tokio::net::TcpListener;

/// Port for binding for the API
const PORT: &str = "127.0.0.1:3001";

/// Combine routes to router
fn get_router() -> Router {
    Router::new().route("/", routing::get(|| async { "HELLO" }))
}

/// Start polling of the API
pub async fn start_polling() -> anyhow::Result<()> {
    let app = get_router();
    let listener = TcpListener::bind(PORT)
        .await
        .context("Failed to run TCP listener")?;

    axum::serve(listener, app).await?;
    Ok(())
}

mod handlers {
    pub async fn load_tasks() {}
    pub async fn add_task() {}
    pub async fn remove_task() {}
    pub async fn load_one_task() {}
    pub async fn complete_task() {}
}
