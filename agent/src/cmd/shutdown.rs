use std::error::Error;
use litcrypt::lc;
use crate::cmd::command_out;

/// Kills the agent.
pub async fn handle() -> Result<String, Box<dyn Error>> {
    command_out!("Shutting down")
}