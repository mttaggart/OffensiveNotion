use crate::config::ConfigOptions;
use serde::{Serialize, Deserialize};
use whoami::username;

/// Categorizes environment checks
#[derive(Debug, Serialize, Deserialize)]
pub enum EnvCheck {
    Username(String),
    Domain(String),
    DomainJoined(bool)
}

impl PartialEq<String> for EnvCheck {
    fn eq(&self, other: &String) -> bool {
        match self {
            EnvCheck::Username(s) => s == other,
            EnvCheck::Domain(s) => s == other,
            _ => false
        }
    }
}

impl PartialEq<bool> for EnvCheck {
    fn eq(&self, other: &bool) -> bool {
        match self {
            EnvCheck::DomainJoined(b) => b == other,
            _ => false
        }
    }
}

/// Validates each kind of `EnvCheck`.
pub fn validate_env(e: &EnvCheck) -> bool {
    match e {
        EnvCheck::Username(u) => {
            let session_username: String = username().to_lowercase();
            #[cfg(target_os = "windows")] {
                // true back in the main method continues the program
                u == session_username.as_str() || session_username ==  "SYSTEM"
            }
            #[cfg(not(windows))] {
                // true back in the main method continues the program
                u == session_username.as_str() || session_username ==  "root"
            }
        },
        // TODO: Implement review for additional EnvChecks.
        _ => true
    }
}

/// Confirms that all environment checks pass
pub async fn check_env_keys(config_options: &ConfigOptions) -> bool {

    config_options.env_checks
    .iter()
    .all(|e| validate_env(e))

}