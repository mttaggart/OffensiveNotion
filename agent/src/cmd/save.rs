use std::error::Error;
use std::fs::write;
use serde_json::to_string as json_to_string;
use crate::cmd::ConfigOptions;

/// Saves the agent to the given path.
/// 
/// Usage: `save [path]`
pub async fn handle(s: &String, config_options: &mut ConfigOptions) -> Result<String, Box<dyn Error>> {
    if !s.is_empty() {
        config_options.config_file_path = s.to_string();
    }
    write(config_options.config_file_path.trim(), json_to_string(config_options)?)?;
    Ok(format!("Config file saved to {s}").to_string())
}