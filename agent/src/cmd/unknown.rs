use std::error::Error;
use litcrypt::lc;
use crate::cmd::command_out;

/// Handles any weirdo commands that can't be interpreted.
pub async fn handle() -> Result<String, Box<dyn Error>> {
    command_out!("[-] Unknown command type")
}