use std::error::Error;
use sysinfo::{ProcessExt, System, SystemExt, User, UserExt};
use whoami::{username};

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
        let mut s = System::new_all();
        let username = username();
        let user = s.users()
        .into_iter()
        .filter(|u| u.name() == username )
        .nth(0)
        .unwrap();
    
        // Uses group membership to determine elevation capabilities
        return user.groups().contains(&"sudo".to_string());
    }

    #[cfg(windows)] {
        false
    }

}

pub async fn handle(s: &String) -> Result<String, Box<dyn Error>> {
    if can_elevate() {
        match s.trim() {
            "sudo" => {
                Ok("Elevating via sudo".to_string())
            },
            _ => Ok("Unknown elevation method".to_string())
        }
    } else {
        Ok("Elevation unavailable".to_string())
    }
}