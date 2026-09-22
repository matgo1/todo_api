use sqlx::PgPool;

use super::{Task, db};
use crate::error_handling::TodoError;

#[derive(Debug, Clone)]
pub struct Todo {
    pub conn: PgPool,
}

impl Todo {
    pub async fn new() -> Self {
        let conn = db::establish_connection().await.unwrap();
        Self { conn }
    }

    /// Add task or raise an error
    pub async fn add_task(
        &self,
        title: String,
        description: Option<String>,
    ) -> Result<Task, TodoError> {
        match db::add(&self.conn, title, description).await {
            // Error handling
            Ok(task) => Ok(task), // Return the task itself if all ok
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(TodoError::AlreadyExist) // Return AlreadyExist if sqlx error of same primary key
            }
            Err(e) => Err(TodoError::Database(e.into())), // In case of another error
        }
    }

    /// Remove task or raise an error
    pub async fn remove_task(&self, title: String) -> Result<Task, TodoError> {
        match db::remove_item(&self.conn, &title).await {
            // If returned not an error
            Ok(task) => match task {
                Some(task) => Ok(task),           // Check if exists
                None => Err(TodoError::NotFound), // Raise and error in the other case
            },
            Err(e) => Err(TodoError::Database(e)), // Raise an unexpected error
        }
    }

    /// Load all items from database
    pub async fn load_all_tasks(&self) -> Result<Vec<Task>, TodoError> {
        match db::load(&self.conn).await {
            Ok(tasks) => Ok(tasks),
            Err(e) => Err(TodoError::Database(e.into())),
        }
    }

    /// Load on item from database
    pub async fn load_task(&self, title: String) -> Result<Task, TodoError> {
        match db::get_item(&self.conn, &title).await {
            // If returned not an error
            Ok(task) => match task {
                Some(task) => Ok(task),           // Check if exists
                None => Err(TodoError::NotFound), // Raise an error in the other case
            },

            Err(e) => Err(TodoError::Database(e)), // Raise an unexpected error
        }
    }

    /// Mark one task as a completed one
    pub async fn complete_task(&self, title: String) -> Result<Task, TodoError> {
        match db::complete_item(&self.conn, &title).await {
            Ok(task) => match task {
                Some(task) => Ok(task),
                None => Err(TodoError::NotFound),
            },
            Err(e) => Err(TodoError::Database(e)),
        }
    }
}
