use super::AppState;
use super::NewTask;
use crate::data_workflow::{Task, db_return_parsing as db};
use crate::error_handling::TodoError;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn load_tasks(State(st): State<AppState>) -> Result<Json<Vec<Task>>, StatusCode> {
    match db::load_all_tasks(&st.conn).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(TodoError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn add_task(
    State(st): State<AppState>,
    Json(payload): Json<NewTask>,
) -> Result<(StatusCode, Json<Task>), StatusCode> {
    match db::add_task(&st.conn, &payload.title, payload.description).await {
        Ok(task) => Ok((StatusCode::CREATED, Json(task))),
        Err(TodoError::AlreadyExist) => Err(StatusCode::CONFLICT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn remove_task(
    State(st): State<AppState>,
    Path(title): Path<String>,
) -> Result<Json<Task>, StatusCode> {
    match db::remove_task(&st.conn, &title).await {
        Ok(task) => Ok(Json(task)),
        Err(TodoError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn load_one_task(
    State(st): State<AppState>,
    Path(title): Path<String>,
) -> Result<Json<Task>, StatusCode> {
    match db::load_task(&st.conn, &title).await {
        Ok(task) => Ok(Json(task)),
        Err(TodoError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn complete_task(
    State(st): State<AppState>,
    Path(title): Path<String>,
) -> Result<Json<Task>, StatusCode> {
    match db::complete_task(&st.conn, &title).await {
        Ok(task) => Ok(Json(task)),
        Err(TodoError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
