//! A minimal REST API backing a Telegram to-do bot.
mod data_workflow;
mod error_handling;
mod router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    router::start_polling().await
}
