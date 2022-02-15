use std::error::Error;

/// Kills the agent.
pub async fn handle() -> Result<String, Box<dyn Error>> {
    Ok("Shutting down".to_string())
}