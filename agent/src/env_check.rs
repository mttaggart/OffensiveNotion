use crate::config::ConfigOptions;
use serde::{Serialize, Deserialize};
use whoami::username;
use whoami::hostname;

#[cfg(target_os = "windows")]
use windows::{
    core::{PSTR, PWSTR, PCWSTR},
    Win32::{
        Foundation::{GetLastError, ERROR_MORE_DATA},
        System::SystemInformation::{GetComputerNameExA, ComputerNameDnsDomain},
        NetworkManagement::NetManagement::{
            NetGetJoinInformation,
            NetApiBufferFree,
            NetSetupUnknownStatus,
            NetSetupDomainName,
        },
    }
};

/// Categorizes environment checks
#[derive(Debug, Serialize, Deserialize)]
pub enum EnvCheck {
    Username(String),
    Hostname(String),
    Domain(String),
    DomainJoined(bool)
}

impl PartialEq<String> for EnvCheck {
    fn eq(&self, other: &String) -> bool {
        match self {
            EnvCheck::Username(s) => s == other,
            EnvCheck::Hostname(s) => s == other,
            EnvCheck::Domain(s) => s == other,
            _ => false
        }
    }
}

impl PartialEq<bool> for EnvCheck {
    fn eq(&self, other: &bool) -> bool {
        match self {
            EnvCheck::DomainJoined(b) => b == other,
            _ => false
        }
    }
}

#[cfg(target_os = "windows")]
/// Get the joined domain name
fn get_domain_name() -> Option<String> {
    let mut domain_name_len = 0;
    // Get the domain name length
    unsafe { GetComputerNameExA(ComputerNameDnsDomain, PSTR(std::ptr::null_mut()), &mut domain_name_len) };

    // GetComputerNameExW will set GetLastError to ERROR_MORE_DATA when querying
    // for domain name length. Domain name lengths of 1 mean that the machine is
    // not joined to a domain.
    if unsafe { GetLastError() } != ERROR_MORE_DATA || domain_name_len <= 1 {
        return None;
    }

    // Get the domain name
    let mut domain_name = vec![0u8; domain_name_len.try_into().ok()?];
    unsafe { GetComputerNameExA(ComputerNameDnsDomain, PSTR(domain_name.as_mut_ptr()), &mut domain_name_len) }.ok().ok()?;

    let str_domain: String = std::str::from_utf8(&domain_name).ok()?.to_string();
    //println!("[*] Domain name: {}", &str_domain.to_string());

    Some(str_domain)
}

#[cfg(target_os = "windows")]
/// Check if the machine is joined to a domain
fn is_domain_joined() -> bool {
    let mut join_status = NetSetupUnknownStatus;
    let mut name_buffer = PWSTR(std::ptr::null_mut());

    // Check the domain join information
    if unsafe {
        NetGetJoinInformation(PCWSTR(std::ptr::null_mut()), &mut name_buffer, &mut join_status)
    } != 0 {
        return false;
    }

    // Free the buffer that `NetGetJoinInformation` allocated
    unsafe { NetApiBufferFree(name_buffer.0.cast()) };

    // Return true if the machine is joined to a domain
    join_status == NetSetupDomainName
}


/// Get the joined domain name
// Sure there are ways linux hosts can be domain joined, but let's focus on the 80% solution here ;)
#[cfg(not(windows))]
fn get_domain_name() -> Option<String> {
    None
}
#[cfg(not(windows))]
/// Get the joined domain name
#[cfg(not(windows))]
fn is_domain_joined() -> bool {
    false
}

/// Validates each kind of `EnvCheck`.
pub fn validate_env(e: &EnvCheck) -> bool {
    match e {
        EnvCheck::Username(u) => {
            let session_username: String = username().to_lowercase();
            #[cfg(target_os = "windows")] {
                // true back in the main method continues the program
                u == session_username.as_str() || session_username ==  "SYSTEM"
            }
            #[cfg(not(windows))] {
                // true back in the main method continues the program
                u == session_username.as_str() || session_username ==  "root"
            }
        },
        EnvCheck::Hostname(h) => {
            let session_hostname: String = hostname().to_lowercase();
            h == session_hostname.as_str()
        },
        EnvCheck::Domain(d) => {
            if let Some(ref domain_name) = get_domain_name() {
                //println!("[*] Keying domain: {}", &d.to_string());
                //println!("[*] Domain name: {}", &domain_name.to_string());
                
                // This means that a substring match for this check will pass 
                if domain_name.to_lowercase().trim().contains (d.to_lowercase().trim()) {
                    true
                } else {
                    false
                } 
            
            } else {
                false
            }
        },
        
        EnvCheck::DomainJoined(j) => {
            j == &is_domain_joined()
    },

        // TODO: Implement review for additional EnvChecks.
        _ => true
    }
}

/// Confirms that all environment checks pass
pub async fn check_env_keys(config_options: &ConfigOptions) -> bool {

    config_options.env_checks
    .iter()
    .all(|e| validate_env(e))

}