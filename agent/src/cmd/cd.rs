use std::error::Error;
use std::path::Path;
use std::env::set_current_dir;

/// Changes the directory using system tools
/// Rather than the shell
pub fn handle(s: &String) -> Result<String, Box<dyn Error>> {
    let new_path = Path::new(s.trim());
    match set_current_dir(new_path) {
        Ok(_) => Ok(format!("Changed to {s}").to_string()),
        Err(e) => Ok(e.to_string())
    }
}

