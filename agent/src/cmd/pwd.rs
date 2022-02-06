use std::error::Error;
use std::env::current_dir;

pub async fn handle(s: String) -> Result<String, Box<dyn Error>> {
    match current_dir() {
        Ok(b) => Ok(String::from(b.to_str().unwrap())),
        Err(e) => Ok(format!("{e}").to_string())
    }
}