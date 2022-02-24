use std::error::Error;
use std::io::copy;
use reqwest::Client;
use std::fs::File;
use crate::logger::Logger;

/// Downloads a file to the local system.
/// 
/// Usage: `download [url] [path]`.
/// 
/// Defaults the the end of the URL without path option
pub async fn handle(s: &String, logger: &Logger) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    // Get args
    let mut args = s.trim().split(" ");
    // Get URL as the first arg
    let url = args.nth(0).unwrap_or_else(|| "");
    // Get path as the 2nd arg or the last part of the URL
    let path = args.nth(0).unwrap_or_else(|| url.split("/").last().unwrap());
    logger.debug(format!("Downloading from {url} to {path}"));
    let r = client.get(url).send().await?;
    if r.status().is_success() {
        if let Ok(mut out_file) = File::create(path) {
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