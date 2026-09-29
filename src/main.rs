//! A minimal REST API backing a Telegram to-do bot.
mod api;
mod data_workflow;
mod error_handling;
use api::router;
pub use error_handling::TodoError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    router::start_polling().await
}
