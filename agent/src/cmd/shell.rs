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
pub async fn handle(cmd_args: &mut CommandArgs) -> Result<String, Box<dyn Error>> {

    let output: std::process::Output;
    #[cfg(windows)] {
        output = Command::new("cmd")
            .arg("/c")
            .arg(cmd_args.to_string())
            .output()
            .expect("failed to execute process");
    }

    #[cfg(target_os = "linux")] {
        output = Command::new("/bin/bash")
            .arg("-c")
            .arg(cmd_args.to_string())
            .output()
            .expect("failed to execute process");
    }

    #[cfg(target_os = "macos")] {
        output = Command::new("/bin/zsh")
            .arg("-c")
            .arg(cmd_args.to_string())
            .output()
            .expect("failed to execute process");
    }

    let output_string: String;
    if output.stderr.len() > 0 {
        output_string = String::from_utf8(output.stderr).unwrap();
    } else {
        output_string = String::from_utf8(output.stdout).unwrap();
    }
    Ok(output_string)
}