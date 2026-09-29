mod handlers;
pub mod router;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
}
