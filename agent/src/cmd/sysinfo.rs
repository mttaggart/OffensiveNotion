use std::error::Error;
use litcrypt::lc;
use whoami::{desktop_env, devicename, distro};
use crate::cmd::notion_out;


/// Returns a whole bunch of info about the current session, leans heavily on the whoami crate and organizes the info
pub async fn handle() -> Result<String, Box<dyn Error>> {
    
    let mut return_string: String = "".to_string();

    let desktop_env = desktop_env().to_string();

    let device_name = devicename().to_string();

    let distro = distro().to_string();

   
    return_string.push_str(&device_name);
    return_string.push_str("\n");
    return_string.push_str(&desktop_env);
    return_string.push_str("\n");
    return_string.push_str(&distro);

    Ok(return_string)
}