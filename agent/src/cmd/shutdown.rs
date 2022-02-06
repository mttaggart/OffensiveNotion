use std::error::Error;

pub async fn handle() -> Result<String, Box<dyn Error>> {
    Ok("Shutting down".to_string())
}