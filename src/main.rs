//! A minimal REST API backing a Telegram to-do bot.
mod logic;
use axum::{Router, routing};
use tokio::net::TcpListener;

/// Port for binding for the API
const PORT: &str = "127.0.0.1:3001";

fn get_router() -> Router {
    Router::new().route("/", routing::get(|| async { "HELLO" }))
}

#[tokio::main]
async fn main() {
    // Get all routers
    let router = get_router();

    // Binding port for the API
    let listener = TcpListener::bind(PORT).await.unwrap();

    let _ = axum::serve(listener, router).await;
}
