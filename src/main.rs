extern crate reqwest;
extern crate tokio;
extern crate serde_json;

use std::error::Error;
// use std::io;
use std::collections::HashMap;
use std::io::{self, Read, BufRead, Write};

use tokio::task;
use serde_json::{json};

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

async fn create_page(client: Client, config_options: ConfigOptions, hostname: String) -> Option<String> {
    println!("Creating page...");
    let url = format!("{}/pages/", URL_BASE);
    
    // Craft JSON Body
    let hn = hostname::get().ok()?;
    let body: serde_json::Value = json!({
        "parent": {
            "type": "page_id",
            "page_id": config_options.parent_page_id
        },
        "properties": {
            "title": [{
                "text": {
                    "content": hostname
                }
            }]
        }
    });
    let r = client
        .post(url)
        .json(&body)
        .send()
        .await
        .unwrap();
    
    if r.status().is_success() {
        let res_body = r.json::<serde_json::Value>().await.unwrap();
        return Some(String::from(res_body["id"].as_str()?));
    }
    println!("{}",r.text().await.unwrap());
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("Starting!");
    
    let config_options_handle = tokio::spawn( async {
        return getConfigOptions();
        
    });
    let config_options = config_options_handle.await?.unwrap();
    let hn = hostname::get()
        .ok()
        .unwrap()
        .into_string()
        .unwrap();
    println!("{:?}", hn);
    println!("{:?}", config_options);
    let mut headers = HeaderMap::new();
    headers.insert("Notion-Version", "2021-08-16".parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert(AUTHORIZATION, format!("Bearer {}", config_options.api_key).parse()?);
    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let page_id = create_page(client, config_options, hn)
    .await
    .unwrap();

    Ok(())
}
