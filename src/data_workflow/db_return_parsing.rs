use sqlx::PgPool;

use super::{Task, db};
use crate::error_handling::TodoError;

/// Add task or raise an error
pub async fn add_task(
    conn: &PgPool,
    title: &str,
    description: Option<String>,
) -> Result<Task, TodoError> {
    match db::add(conn, title, description).await {
        // Error handling
        Ok(task) => Ok(task), // Return the task itself if all ok
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(TodoError::AlreadyExist) // Return AlreadyExist if sqlx error of same primary key
        }
        Err(e) => Err(TodoError::Database(e.into())), // In case of another error
    }
}

/// Remove task or raise an error
pub async fn remove_task(conn: &PgPool, title: &str) -> Result<Task, TodoError> {
    match db::remove_item(conn, title).await {
        // If returned not an error
        Ok(task) => match task {
            Some(task) => Ok(task),           // Check if exists
            None => Err(TodoError::NotFound), // Raise and error in the other case
        },
        Err(e) => Err(TodoError::Database(e)), // Raise an unexpected error
    }
}

/// Load all items from database
pub async fn load_all_tasks(conn: &PgPool) -> Result<Vec<Task>, TodoError> {
    match db::load(conn).await {
        Ok(tasks) => Ok(tasks),
        Err(e) => Err(TodoError::Database(e)),
    }
}

/// Load on item from database
pub async fn load_task(conn: &PgPool, title: &str) -> Result<Task, TodoError> {
    match db::get_item(conn, title).await {
        // If returned not an error
        Ok(task) => match task {
            Some(task) => Ok(task),           // Check if exists
            None => Err(TodoError::NotFound), // Raise an error in the other case
        },

        Err(e) => Err(TodoError::Database(e)), // Raise an unexpected error
    }
}

/// Mark one task as a completed one
pub async fn complete_task(conn: &PgPool, title: &str) -> Result<Task, TodoError> {
    match db::complete_item(conn, title).await {
        Ok(task) => match task {
            Some(task) => Ok(task),
            None => Err(TodoError::NotFound),
        },
        Err(e) => Err(TodoError::Database(e)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn add_then_return_same_task(pool: PgPool) {
        let title = "Test";
        let description = Some("Test".to_string());
        let init_task = add_task(&pool, title, description).await.unwrap();

        let returned_task = load_task(&pool, title).await.unwrap();

        assert_eq!(init_task, returned_task);
    }
}
