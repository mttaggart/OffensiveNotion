use std::error::Error;
use litcrypt::lc;
use crate::cmd::{CommandArgs, notion_out};
use crate::logger::{Logger, log_out};

use std::fs;
use std::path::{Path, PathBuf};

pub async fn handle(cmd_args: &mut CommandArgs, logger: &Logger)-> Result<String, Box<dyn Error>> {
    
    let file_vector: Vec<String> = std::fs::read_dir(".").unwrap() 
        .filter_map(|maybe_dir_entry| {
            let dir_entry = maybe_dir_entry.ok()?;
            let path_buf = dir_entry.path();
            let file_name = path_buf.file_name()?;
            let string = file_name.to_str()?; 
            Some(string.to_string())
        })
        .collect();

        let mut return_string: String = "".to_string();

        for file_name in file_vector {
            return_string.push_str(&file_name);
            return_string.push_str("\n");
        }

        if return_string.is_empty() {
            Ok(lc!("[*] Directory is empty"))
        } else {
            Ok(return_string)
        }
}