use std::error::Error;
use whoami::username;

/// Gives the username. That's it; that's the func.
pub async fn handle() -> Result<String, Box<dyn Error>> {
    Ok(username())
}