//! REST API handling
use anyhow::Context;
use axum::{Router, routing::*};

use tokio::net::TcpListener;

use crate::data_workflow::db::establish_connection;

use super::AppState;
use super::handlers;

/// Port for binding for the API
const PORT: &str = "127.0.0.1:3001";

/// Combine routes to router
async fn app() -> anyhow::Result<Router> {
    // Create state
    let conn = establish_connection()
        .await
        .context("Failed to connect to the db")?;
    let state: AppState = AppState { conn };

    Ok(Router::new()
        .route("/task", get(handlers::load_tasks))
        .route("/task/{title}", get(handlers::load_one_task))
        .route("/task", post(handlers::add_task))
        .route("/task/{title}/delete", delete(handlers::remove_task))
        .route("/task/{title}/complete", patch(handlers::complete_task))
        .with_state(state))
}

/// Start polling of the API
pub async fn start_polling() -> anyhow::Result<()> {
    let app = app().await?;
    let listener = TcpListener::bind(PORT)
        .await
        .context("Failed to run TCP listener")?;
    axum::serve(listener, app).await?;
    Ok(())
}
