use serde::Serialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use time::OffsetDateTime;

/// Database's unit structure
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Task {
    /// Name of which task
    /// Key value
    pub title: String,
    /// Optional information to task
    pub description: Option<String>,
    /// Time it's created
    pub created_at: OffsetDateTime,
    /// Time it's completed (if completed)
    pub completed_at: Option<OffsetDateTime>,
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
    title: &str,
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
pub async fn get_item(conn: &PgPool, title: &str) -> anyhow::Result<Option<Task>> {
    let task = sqlx::query_as!(
        Task,
        "SELECT title, description, created_at, completed_at FROM tasks WHERE title = $1",
        title
    )
    .fetch_optional(conn)
    .await?;

    Ok(task)
}

/// Remove task by its title
pub async fn remove_item(conn: &PgPool, title: &str) -> anyhow::Result<Option<Task>> {
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
pub async fn complete_item(conn: &PgPool, title: &str) -> anyhow::Result<Option<Task>> {
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

// TESTS
#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn add_then_return_same_task(pool: PgPool) {
        let description = Some("Test desc".into());

        add(&pool, "Test", description).await.unwrap();

        let task = get_item(&pool, "Test")
            .await
            .unwrap()
            .expect("Expected the task to be found");

        assert_eq!(task.title, "Test");
        assert_eq!(task.description.as_deref(), Some("Test desc"));
    }

    #[sqlx::test]
    async fn add_without_description_stores_none(pool: PgPool) {
        add(&pool, "Test", None).await.unwrap();

        let task = get_item(&pool, "Test")
            .await
            .unwrap()
            .expect("Expected the task to be found");

        assert_eq!(task.title, "Test");
        assert_eq!(task.description, None);
    }

    #[sqlx::test]
    async fn remove_deletes_row(pool: PgPool) {
        add(&pool, "Test", None).await.unwrap();

        let task = get_item(&pool, "Test")
            .await
            .unwrap()
            .expect("Expected the task to be found");

        assert_eq!(task.title, "Test");
        assert_eq!(task.description, None);

        let deleted_task: Option<Task> = remove_item(&pool, "Test").await.unwrap();

        assert!(deleted_task.is_some(), "Expected to be found");

        let task_shouldnot_found: Option<Task> = get_item(&pool, "Test").await.unwrap();

        assert!(task_shouldnot_found.is_none());
    }

    #[sqlx::test]
    async fn load_all_tasks_returns_some(pool: PgPool) {
        add(&pool, "Test1", None).await.unwrap();
        add(&pool, "Test2", None).await.unwrap();

        let tasks: Vec<Task> = load(&pool).await.unwrap();

        assert_eq!(tasks.len(), 2)
    }

    #[sqlx::test]
    async fn completed_changes_state(pool: PgPool) {
        add(&pool, "Test1", None).await.unwrap();

        let task = get_item(&pool, "Test1")
            .await
            .unwrap()
            .expect("Expected the task to be found");

        assert!(task.completed_at.is_none());

        complete_item(&pool, "Test1").await.unwrap();

        let task = get_item(&pool, "Test1")
            .await
            .unwrap()
            .expect("Expected the task to be found");

        assert!(task.completed_at.is_some())
    }
}
