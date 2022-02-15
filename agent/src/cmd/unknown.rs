use std::error::Error;

/// Handles any weirdo commands that can't be interpreted.
pub async fn handle() -> Result<String, Box<dyn Error>> {
    Ok("[-] Unknown command type".to_string())
}