use std::error::Error;
use std::path::Path;
use std::env::set_current_dir;

/// Changes the directory using system tools
/// Rather than the shell
pub fn handle(cmd_args: Vec<String>) -> Result<String, Box<dyn Error>> {
    let path_arg = cmd_args[0];
    let new_path = Path::new(&path_arg);
    match set_current_dir(new_path) {
        Ok(_) => Ok(format!("Changed to {path_arg}").to_string()),
        Err(e) => Ok(e.to_string())
    }
}

