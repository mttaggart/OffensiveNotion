use std::process::Command;
use std::error::Error;
use crate::cmd::CommandArgs;

/// Executes the given shell command.
/// 
/// On Windows, calls out to `cmd.exe`.
/// 
/// On Linux, calls out to `/bin/bash`.
/// 
/// Usage: `shell [command]`
pub async fn handle(cmd_args: CommandArgs) -> Result<String, Box<dyn Error>> {
    let args_vec: Vec<String> = cmd_args.collect();
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .args(args_vec)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("/bin/bash")
            .arg("-c")
            .args(args_vec)
            .output()
            .expect("failed to execute process")
    };
    let output_string: String;
    if output.stderr.len() > 0 {
        output_string = String::from_utf8(output.stderr).unwrap();
    } else {
        output_string = String::from_utf8(output.stdout).unwrap();
    }
    return Ok(output_string);
}