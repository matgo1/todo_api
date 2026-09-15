//! A minimal REST API backing a Telegram to-do bot.
mod logic;
use axum::{Router, routing};
use tokio::net::TcpListener;

/// Port for binding for the API
const PORT: &str = "127.0.0.1:3001";

#[tokio::main]
async fn main() {
    // Get all routers
    let app = Router::new().route("/", routing::get(|| async { "HELLO" }));

    // Binding port for the API
    let listener = TcpListener::bind(PORT).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
