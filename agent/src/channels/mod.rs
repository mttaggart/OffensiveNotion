use std::{fmt::{Display, self}};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub struct ChannelError {
    msg: String
}

impl Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}


///
/// Handles the communication to the trusted sites.
/// 
/// All `Channel`s have 4 required methods.
/// 
/// * `init()`: Set up the `Channel` to communicate
/// * `send()`: Send data over the `Channel`
/// * `receive()`: Receive dat from the `Channel`
/// * `update()`: Update the `Channel` configuration.
/// 
/// NOTE FOR ME: There is no good reason to overcomplicate this. Send JSON directly
/// to the module and let its struct handle type safety. You won't know whether it's good
/// data until the `init()` runs anyway, so why bother with the enum? 
/// Send it a String and call it a day
/// 
pub trait Channel {
    fn init(&self, options: String) -> Result<Channel, ChannelError>;
    fn send(&self, data: String) -> Result<String, ChannelError>;
    fn receive(&self) -> Result<Value, ChannelError>;
    fn to_base64(&self) -> String;
    fn update(&self, options: String) -> Result<String, ChannelError>;
}
