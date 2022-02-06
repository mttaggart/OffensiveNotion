use std::error::Error;
use std::env::current_dir;

pub async fn handle(s: String) -> Result<String, Box<dyn Error>> {
    if !s.is_empty() {
        config_options.config_file_path = s.to_string();
    }
    write(config_options.config_file_path.trim(), json_to_string(config_options)?)?;
    Ok(format!("Config file saved to {s}").to_string())
}