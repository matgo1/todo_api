pub mod handlers;
pub mod router;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub conn: PgPool,
}
