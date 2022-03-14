use std::error::Error;
use std::fs::write;
use serde_json::to_string as json_to_string;
// use relative_path::RelativePath;
use crate::cmd::{CommandArgs, ConfigOptions, notion_out};

/// Saves the agent to the given path.
/// 
/// Usage: `save [path]`
pub async fn handle(cmd_args: &mut CommandArgs, config_options: &mut ConfigOptions) -> Result<String, Box<dyn Error>> {
    let save_path = cmd_args.nth(0).unwrap_or_else(|| config_options.config_file_path.to_owned());
    config_options.config_file_path = save_path.to_owned();
    // let write_path = RelativePath::new(config_options.config_file_path.as_str());
    match write(&config_options.config_file_path, json_to_string(config_options)?) {
        Ok(_) => notion_out!("Config file saved to {save_path}"),
        Err(e) => Ok(format!("{e}"))
    }
}