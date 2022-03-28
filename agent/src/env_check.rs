use std::error::Error;
use crate::config::ConfigOptions;
use serde::{Serialize, Deserialize};
use serde_json::{Value,};
use whoami::username;


#[derive(Debug, Serialize, Deserialize)]
pub enum EnvCheckType {
    Username,
    Domain,
    Processors
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EnvCheckValue {
    String(String),
    Number(u64)
}

impl PartialEq<String> for EnvCheckValue {
    fn eq(&self, other: &String) -> bool {
        match self {
            EnvCheckValue::String(s) => s == other,
            _ => false
        }
    }
}

impl PartialEq<u64> for EnvCheckValue {
    fn eq(&self, other: &u64) -> bool {
        match self {
            EnvCheckValue::Number(n) => n == other,
            _ => false
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvCheck {
    check_type: EnvCheckType,
    value: EnvCheckValue
}

pub fn validate_env(e: &EnvCheck) -> bool {
    match e.check_type {
        EnvCheckType::Username => {
            let session_username: String = username().to_lowercase();
            #[cfg(target_os = "windows")] {
                // true back in the main method continues the program
                e.value == session_username || session_username ==  "SYSTEM"
            }
            #[cfg(not(windows))] {
                // true back in the main method continues the program
                e.value == session_username || session_username ==  "root"
            }
        },
        _ => true
    }
}


// Naive approach

pub async fn check_env_keys(config_options: &ConfigOptions) -> bool {

    config_options.env_checks
    .iter()
    .all(|e| validate_env(e))
    // Marshal and check your configs
    // Evaluate if there are any keys to check against. If there are no keys set, return from this function and continue with the program.

    // if config_options.key_username == "" {
    //     println!("[+] No username key set. Continuing program...");
    //     return true
    // }

    // let key_username = config_options.key_username.to_lowercase();

    // println!("[+] Keying username: {}", config_options.key_username);

    // // But if there are configs to check against, perform the required checks. If they pass, return from the program and carry on.

    // let session_username: String = username().to_lowercase();

    // println!("[+] Session username: {}", username());
    
    // // If the checks fail, kill the program outright.
    

}