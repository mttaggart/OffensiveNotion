use std::error::Error;
use std::io::copy;
use reqwest::Client;
use std::fs::File;
use crate::cmd::CommandArgs;
use crate::logger::Logger;

/// Downloads a file to the local system.
/// 
/// Usage: `download [url] [path]`.
/// 
/// Defaults the the end of the URL without path option
pub async fn handle(cmd_args: &mut CommandArgs, logger: &Logger) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    // Get URL as the first arg
    let url: String  = cmd_args.nth(0).unwrap_or_else(|| "".to_string());
    // Get path as the 2nd arg or the last part of the URL
    let path: String = cmd_args.nth(0).unwrap_or_else(|| url.split("/").last().unwrap().to_string());
    logger.debug(format!("Downloading from {url} to {path}"));
    let r = client.get(url).send().await?;
    if r.status().is_success() {
        if let Ok(mut out_file) = File::create(&path) {
            match copy(&mut r.bytes().await?.as_ref(), &mut out_file) {
                Ok(b)  => { return Ok(format!("{b} bytes written to {path}").to_string());},
                Err(_) => { return Ok("Could not write file".to_string()); }
            }
        } else {
            return Ok("Could not write file".to_string());
        }
    }
    Ok(r.text().await?)
}