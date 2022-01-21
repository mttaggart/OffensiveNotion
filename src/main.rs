extern crate reqwest;
use std::io;
use std::collections::HashMap;

use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

const URL_BASE: &str = "https://api.notion.com/v1";
const API_KEY_URL: &str = "http://localhost:8888";

struct ConfigOptions {
    sleep_interval: isize,
    parent_page_id: String,
    api_key: String
}

fn getConfigOptions() -> Result<ConfigOptions, io::Error> {
    
    let mut sleep_interval = String::new();
    print!("[*] Enter agent sleep interval > ");
    io::stdin().read_line(&mut sleep_interval)?;
    match io::stdin().read_line(&mut sleep_interval) {
        Ok(_) => {},
        Err(e) => { return Err(e)}
    }
    let mut parent_page_id = String::new();
    print!("[*] Enter parent page id > ");
    match io::stdin().read_line(&mut parent_page_id) {
        Ok(_) => {},
        Err(e) => { return Err(e)}
    }
    let mut api_key = String::new();
    print!("[*] Enter API Key > ");
    match io::stdin().read_line(&mut api_key) {
        Ok(_) => {},
        Err(e) => { return Err(e)}
    }

    Ok(
        ConfigOptions { 
            sleep_interval: sleep_interval.parse().unwrap(),
            parent_page_id,
            api_key
        }
    )
}

async fn create_page(client: Client, config_options: ConfigOptions) -> Option<&str> {
    
    // Craft JSON Body
    
    Some("")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let config_options = getConfigOptions()?;
    let mut headers = HeaderMap::new();
    headers.insert("Notion-Version", "2021-08-16".parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert(AUTHORIZATION, format!("Bearer {}", config_options.api_key).parse()?);
    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(())
}
