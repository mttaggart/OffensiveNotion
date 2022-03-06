use std::error::Error;
use crate::cmd::CommandArgs;

/// Runs given command as another user. Requires admin privs.
/// 
/// Usage: `runas [user] [command]`
pub async fn handle(cmd_args: CommandArgs) -> Result<String, Box<dyn Error>> {
    // TODO: Implement
    #[cfg(windows)] {
        return Ok(String::from("Under Construction!"))
    }
    #[cfg(not(windows))] {
        return Ok(String::from("Runas only works on Windows!"))
    }
}