use std::error::Error;
use std::env::current_dir;
use crate::cmd::notion_out;

/// Prints working directory.
pub async fn handle() -> Result<String, Box<dyn Error>> {
    match current_dir() {
        Ok(b) => Ok(String::from(b.to_str().unwrap())),
        Err(e) => notion_out!("{e}")
    }
}