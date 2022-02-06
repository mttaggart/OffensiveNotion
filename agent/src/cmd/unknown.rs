use std::error::Error;

pub async fn handle() -> Result<String, Box<dyn Error>> {
    Ok("Unknown command type".to_string())
}