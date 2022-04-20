use std::error::Error;
use litcrypt::lc;
use crate::cmd::notion_out;

/// Handles any weirdo commands that can't be interpreted.
pub async fn handle() -> Result<String, Box<dyn Error>> {
    notion_out!("[-] Unknown command type")
}