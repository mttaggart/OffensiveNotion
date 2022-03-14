use std::error::Error;
use crate::cmd::{CommandArgs, notion_out};

/// Runs given command as another user. Requires admin privs.
/// 
/// Usage: `runas [user] [command]`
pub async fn handle(_cmd_args: &CommandArgs) -> Result<String, Box<dyn Error>> {
    // TODO: Implement
    #[cfg(windows)] {
        return notion_out!("Under Construction!");
    }
    #[cfg(not(windows))] {
        return notion_out!("Runas only works on Windows!");
    }
}