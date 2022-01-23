                                                       use std::error::Error;
use std::io::{self, Write};

pub const URL_BASE: &str = "https://api.notion.com/v1";
pub const DEFAULT_API_KEY: &str = "<<API_KEY>>";
pub const DEFAULT_PARENT_PAGE_ID: &str = "<<PARENT_PAGE_ID>>";
pub const DEFAULT_SLEEP_INTERVAL: u64 = <<SLEEP>>;


/// Storing Config Options as a struct for ergonomics.
///
/// sleep_interval: u64 for use with `std::thread::sleep()`
///
/// parent_page_id: String which eventually can be added at compile
///
/// api_key: String also added at compile
#[derive(Debug)]
pub struct ConfigOptions {
    pub sleep_interval: u64,
    pub parent_page_id: String,
    pub api_key: String
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

    let mut parent_page_id = String::new();
    print!("[*] Enter parent page id > ");
    io::stdout().flush()?;
    stdin.read_line(&mut parent_page_id)?;

    let mut api_key = String::new();
    println!("[*] Enter API Key > ");
    io::stdout().flush()?;
    stdin.read_line(&mut api_key)?;

    Ok(
        ConfigOptions {
            sleep_interval: sleep_interval.trim().parse().unwrap(),
            parent_page_id: String::from(parent_page_id.trim()),
            api_key: String::from(api_key.trim())
        }
    )
}

pub async fn get_config_options() -> Result<ConfigOptions, Box<dyn Error>> {
    let config_options = ConfigOptions {
        sleep_interval: DEFAULT_SLEEP_INTERVAL,
        parent_page_id: String::from(DEFAULT_PARENT_PAGE_ID),
        api_key: String::from(DEFAULT_API_KEY)
    };
    Ok(config_options)
}