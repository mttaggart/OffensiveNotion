use std::error::Error;
use litcrypt::lc;
use whoami::{desktop_env, devicename, distro, username, platform, hostname};
use crate::cmd::command_out;


/// Returns a whole bunch of info about the current session, leans heavily on the whoami crate and organizes the info
pub async fn handle() -> Result<String, Box<dyn Error>> {
    
    let mut return_string: String = lc!("================= SYSINFO =================");


    let mut str_username: String  = lc!("❓ Username: ");
    let mut str_hostname: String  = lc!("🏡 Hostname: ");
    let mut str_distro: String = lc!("📀 Distro: ");
    let mut str_platform: String = lc!("🖥️ Platform: ");

    let session_username: String = username();
    let session_hostname: String = hostname();
    let desktop_env = desktop_env().to_string();
    let session_distro = distro().to_string();
    let session_platform: String = platform().to_string();


    str_username.push_str(&session_username);
    str_hostname.push_str(&session_hostname);
    str_distro.push_str(&session_distro);
    str_platform.push_str(&session_platform);

    return_string.push_str("\n");
    return_string.push_str(&str_username);
    return_string.push_str("\n");
    return_string.push_str(&str_hostname);
    return_string.push_str("\n");
    return_string.push_str(&str_distro);
    return_string.push_str("\n");
    return_string.push_str(&str_platform);


    Ok(return_string)
}