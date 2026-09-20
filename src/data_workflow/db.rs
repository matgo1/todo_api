use anyhow::Ok;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use time::OffsetDateTime;

/// Database's unit structure
#[derive(Debug)]
pub struct Task {
    /// Name of which task
    /// Key value
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

/// create connection
pub async fn establish_connection() -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .idle_timeout(std::time::Duration::from_secs(10))
        .connect(&get_url()?)
        .await?;

    Ok(pool)
}

/// Add a task into db or return a unique for this function error
pub async fn add(
    conn: &PgPool,
    title: String,
    description: Option<String>,
) -> Result<Task, sqlx::Error> {
    sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title, description) VALUES ($1, $2)
        RETURNING title, description, created_at, completed_at",
        title,
        description,
    )
    .fetch_one(conn)
    .await
}

/// Get all database
pub async fn load(conn: &PgPool) -> anyhow::Result<Vec<Task>> {
    let task = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(conn)
        .await?;
    Ok(task)
}

/// Get one task
pub async fn get_item(conn: &PgPool, title: &String) -> anyhow::Result<Option<Task>> {
    let task = sqlx::query_as!(
        Task,
        "SELECT title, description, created_at, completed_at FROM tasks WHERE title = $1",
        *title
    )
    .fetch_optional(conn)
    .await?;

    Ok(task)
}

/// Remove task by its title
pub async fn remove_item(conn: &PgPool, title: &String) -> anyhow::Result<Option<Task>> {
    let task = sqlx::query_as!(
        Task,
        "DELETE FROM tasks WHERE title = $1
        RETURNING title, description, created_at, completed_at",
        title
    )
    .fetch_optional(conn)
    .await?;

    Ok(task)
}

/// Mark task as completed
pub async fn complete_item(conn: &PgPool, title: &String) -> anyhow::Result<Option<Task>> {
    let task = sqlx::query_as!(
        Task,
        "UPDATE tasks SET completed_at = now() WHERE title = $1
        RETURNING title, description, created_at, completed_at",
        title
    )
    .fetch_optional(conn)
    .await?;

    Ok(task)
}
