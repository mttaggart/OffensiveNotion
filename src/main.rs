extern crate reqwest;
extern crate tokio;
extern crate text_io;
use std::error::Error;
// use std::io;
use std::collections::HashMap;
use std::io::{self, Read, BufRead, Write};

use tokio::task;
// use tokio::io::{Error};

use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

const URL_BASE: &str = "https://api.notion.com/v1";
const API_KEY_URL: &str = "http://localhost:8888";

#[derive(Debug)]
struct ConfigOptions {
    sleep_interval: isize,
    parent_page_id: String,
    api_key: String
}

fn getConfigOptions() -> Result<ConfigOptions, Box<dyn Error + Send + Sync>> {
   
    println!("Getting config options!");
    let mut stdin = std::io::stdin();
    
    println!("Getting sleep");

    let mut sleep_interval = String::new();
    print!("[*] Enter agent sleep interval > ");
    io::stdout().flush()?;
    stdin.read_line(&mut sleep_interval)?;

    println!("Getting parent");
    
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

async fn create_page(client: Client, config_options: ConfigOptions) -> Option<String> {
    println!("Getting hostname");
    // Craft JSON Body
    let hn = hostname::get().ok()?;
    
    println!("{:?}", hn);
    Some(hn.into_string().unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("Starting!");
    
    let config_options_handle = tokio::spawn( async {
        return getConfigOptions();
        
    });
    let config_options = config_options_handle.await?.unwrap();
    let hn = hostname::get().ok().unwrap();
    println!("{:?}", hn);
    println!("{:?}", config_options);
    let mut headers = HeaderMap::new();
    headers.insert("Notion-Version", "2021-08-16".parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert(AUTHORIZATION, format!("Bearer {}", config_options.api_key).parse()?);
    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let page_id = create_page(client, config_options)
    .await
    .unwrap();

    Ok(())
}
