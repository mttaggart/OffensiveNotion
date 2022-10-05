#[cfg(windows)] use windows::{
    Win32::{
        Foundation::{
            BOOL,
        },
        Security::{
            RevertToSelf
        }
    }
};
#[cfg(windows)] use whoami;
use std::error::Error;
use litcrypt::lc;
use crate::logger::{Logger, log_out};
use crate::cmd::command_out;

/// Reverts to self if impersonated
pub async fn handle() -> Result<String, Box<dyn Error>> {
    
    #[cfg(windows)] {
        let username = whoami::username();
        if username == "SYSTEM" {
            unsafe {
                if RevertToSelf().0 == 1 {
                    return command_out!("Reverted to Self: ", whoami::username().as_str());
                } else {
                    return command_out!("Could not revert");
                }
            }
        }
        command_out!("Not SYSTEM, no reason to revert!")
    }
    
    #[cfg(not(windows))] {
        command_out!("This module only works on Windows!")
    }
}