use std::error::Error;
use std::io::{self, Write};
use std::fs;
use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_string, to_value};
use base64::encode;

// Config consts
pub const URL_BASE: &str = "https://api.notion.com/v1";
pub const DEFAULT_API_KEY: &str = "<<API_KEY>>";
pub const DEFAULT_PARENT_PAGE_ID: &str = "<<PARENT_PAGE_ID>>";
pub const DEFAULT_SLEEP_INTERVAL: &str = "<<SLEEP>>";
pub const DEFAULT_JITTER_TIME: &str = "<<JITTER>>";
pub const DEFAULT_LOG_LEVEL: &str = "<<LOG_LEVEL>>";
pub const CONFIG_FILE_PATH: &str = "./cfg.json";

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
    pub sleep_interval: u64,
    pub jitter_time: u64,
    pub parent_page_id: String,
    pub api_key: String,
    pub config_file_path: String,
    pub launch_app: bool,
    pub log_level: u64
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

    /// Converts loaded json data into `ConfigOptions`
    pub fn from_json(j: Value) -> ConfigOptions {
        println!("{:?}", j);
        ConfigOptions {
            sleep_interval: j["sleep_interval"].as_u64().unwrap(),
            jitter_time: j["jitter_time"].as_u64().unwrap(),
            parent_page_id: j["parent_page_id"].to_string().replace('"', ""),
            api_key: j["api_key"].to_string().replace('"', ""),
            config_file_path: j["config_file_path"].to_string().replace('"', ""),
            launch_app: j["launch_app"].as_bool().unwrap_or_default(),
            log_level: j["log_level"].as_u64().unwrap_or_else(|| 4) // Info as default log level
        }
    }

    /// Produces the Jsonified version of the ConfigOptions
    pub fn to_json(&self) -> Value {
        to_value(self).unwrap()
    }

    /// Produces a base64 encoded String of the Options.
    ///
    /// This is useful for sending ConfigOptions to launch commands
    pub fn to_base64(&self) -> String {
        encode(to_string(self).unwrap().as_bytes())
    }

}

/// Retrieves config options from the terminal.
///
/// This is tricky because the terminal doesn't async in a normal way. That's why
/// it's invoked with a tokio::spawn to encapsulate the work in an async thread.
pub fn get_config_options_debug() -> Result<ConfigOptions, Box<dyn Error + Send + Sync>> {

    println!("Getting config options!");
    let stdin = std::io::stdin();

    let mut sleep_interval = String::new();
    print!("[*] Enter agent sleep interval > ");
    io::stdout().flush()?;
    stdin.read_line(&mut sleep_interval)?;

    let mut jitter_time = String::new();
    print!("[*] Enter agent jitter time > ");
    io::stdout().flush()?;
    stdin.read_line(&mut jitter_time)?;

    let mut parent_page_id = String::new();
    print!("[*] Enter parent page id > ");
    io::stdout().flush()?;
    stdin.read_line(&mut parent_page_id)?;

    let mut api_key = String::new();
    println!("[*] Enter API Key > ");
    io::stdout().flush()?;
    stdin.read_line(&mut api_key)?;

    let mut config_file_path = String::new();
    println!("[*] Enter Config File Path > ");
    io::stdout().flush()?;
    stdin.read_line(&mut config_file_path)?;

    let mut log_level = String::new();
    println!("[*] Enter Log Level (1-4) > ");
    io::stdout().flush()?;
    stdin.read_line(&mut log_level)?;

    Ok(
        ConfigOptions {
            sleep_interval: sleep_interval.trim().parse().unwrap(),
            jitter_time: jitter_time.trim().parse().unwrap(),
            parent_page_id: parent_page_id.trim().to_string(),
            api_key: api_key.trim().to_string(),
            config_file_path: config_file_path.trim().to_string(),
            launch_app: false,
            log_level: log_level.trim().parse().unwrap(),
        }
    )
}

/// Sets default config options from defined constants.
pub async fn get_config_options() -> Result<ConfigOptions, ConfigError> {
    let config_options = ConfigOptions {
        sleep_interval: DEFAULT_SLEEP_INTERVAL.parse().unwrap_or_else(|_| 10),
        jitter_time: DEFAULT_JITTER_TIME.parse().unwrap_or_else(|_| 0),
        parent_page_id: DEFAULT_PARENT_PAGE_ID.to_string(),
        api_key: DEFAULT_API_KEY.to_string(),
        config_file_path: CONFIG_FILE_PATH.to_string(),
        launch_app: true,
        log_level: DEFAULT_LOG_LEVEL.parse().unwrap_or_else(|_| 2),
    };
    
    Ok(config_options)
}

/// Ingests config from a saved JSON file—or tries to.
/// 
/// If `None` is passed as the path, the `CONFIG_FILE_PATH` is attempted.
/// 
/// If no config file can be parsed, defaults are used.
pub async fn load_config_options(c: Option<&str>) -> Result<ConfigOptions, ConfigError> {

    let config_file_path = match c {
        Some(p) => p,
        None => CONFIG_FILE_PATH
    };

    if let Ok(c) = fs::read_to_string(config_file_path) {
        if let Ok(cfg) = serde_json::from_str(c.as_str()) {
            Ok(ConfigOptions::from_json(cfg))
        } else {
            println!("[!] Could not convert {c} to JSON");
            get_config_options().await
        }
    } else {
        get_config_options().await
    }
}
