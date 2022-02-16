use std::error::Error;
use std::fs::write;
use serde_json::to_string as json_to_string;
// use relative_path::RelativePath;
use crate::cmd::ConfigOptions;

/// Saves the agent to the given path.
/// 
/// Usage: `save [path]`
pub async fn handle(s: &String, config_options: &mut ConfigOptions) -> Result<String, Box<dyn Error>> {
    if !s.is_empty() {
        config_options.config_file_path = s.trim().to_string();
    }
    // let write_path = RelativePath::new(config_options.config_file_path.as_str());
    match write(&config_options.config_file_path, json_to_string(config_options)?) {
        Ok(_) => Ok(format!("Config file saved to {s}").to_string()),
        Err(e) => Ok(format!("{e}"))
    }
}