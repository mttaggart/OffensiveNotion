use std::error::Error;
use crate::config::ConfigOptions;
use whoami::username;

// Naive approach

pub async fn check_env_keys(config_options: &mut ConfigOptions) -> bool {
    // Marshal and check your configs
    // Evaluate if there are any keys to check against. If there are no keys set, return from this function and continue with the program.

    if config_options.key_username == "" {
        println!("[+] No username key set. Continuing program...");
        return true
    }

    println!("[+] Keying username: {}", config_options.key_username);

    // But if there are configs to check against, perform the required checks. If they pass, return from the program and carry on.

    let session_username: String = username();

    println!("[+] Session username: {}", username());
    
    // If the checks fail, kill the program outright.
    if session_username == config_options.key_username {
        // true back in the main method continues the program
        return true
    }
    else {
        // false back in the main method kills/selfdestructs the program
        return false
    }
}