use std::error::Error;
use crate::cmd::notion_out;

/// Kills the agent.
pub async fn handle() -> Result<String, Box<dyn Error>> {
    notion_out!("Shutting down")
}