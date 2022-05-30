extern crate encoding;
use encoding::all::GB18030;
use encoding::{DecoderTrap,EncoderTrap,Encoding};
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
        use std::os::windows::process::CommandExt;
        output = Command::new("cmd")
            .creation_flags(0x08000000)
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
        let result1:&[u8] = &output.stderr;
        output_string = GB18030.decode(result1, DecoderTrap::Strict).unwrap_or("Pls check your command or args.".to_string());
    } else {
        let result2:&[u8] = &output.stdout;
        output_string = GB18030.decode(result2, DecoderTrap::Strict).unwrap_or("Some decode error.".to_string());
    }
    Ok(output_string)
}