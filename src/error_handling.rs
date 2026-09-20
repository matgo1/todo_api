/// Enum for errors todo Database working
#[derive(thiserror::Error, Debug)]
pub enum TodoError {
    #[error("Task not found")]
    NotFound,

    #[error("Task already exist")]
    AlreadyExist,

    #[error(transparent)]
    Database(#[from] anyhow::Error),
}
