use anyhow::Result;
use axum::{Router, routing};
use tokio::net::TcpListener;

const PORT: &str = "127.0.0.1:3001";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind(PORT).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
