use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use time::OffsetDateTime;

/// Database's unit structure
#[derive(Debug)]
struct Task {
    /// Id
    id: i64,
    /// Name of which task
    title: String,
    /// Optional information to task
    description: Option<String>,
    /// Time it's created
    created_at: OffsetDateTime,
    /// Time it's completed (if completed)
    completed_at: Option<OffsetDateTime>,
}

/// Get url of db from .env
fn get_url() -> anyhow::Result<String> {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("URL must be set");

    Ok(db_url)
}

// create connection
pub async fn establish_connection() -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .idle_timeout(std::time::Duration::from_secs(10))
        .connect(&get_url()?)
        .await?;

    Ok(pool)
}

// pub async fn db_add(conn: &PgPool, task: Task) -> anyhow::Result<Task> {}
