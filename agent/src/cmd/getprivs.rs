use std::error::Error;
use is_root::is_root;
use litcrypt::lc;
use crate::cmd::notion_out;

#[cfg(windows)] use std::mem;
#[cfg(windows)] use std::ptr::null_mut;
#[cfg(windows)] use std::ffi::c_void;
#[cfg(windows)] use windows::{
    core::{PSTR, PWSTR, PCWSTR},
    Win32::{
        Foundation::{
            CloseHandle,
            HANDLE
        },
        System::Threading::{
            GetCurrentProcess,
            OpenProcessToken
        },
        Security::{
            GetTokenInformation,
            TokenElevation,
            TOKEN_ELEVATION,
            TOKEN_QUERY
        }
    }
};

pub fn is_elevated() -> bool {
    #[cfg(not(windows))] {
        let is_root = is_root();
        return is_root;
    }

    #[cfg(windows)] {
        let mut handle = HANDLE(0);
        unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) };

        let elevation = unsafe { libc::malloc(mem::size_of::<TOKEN_ELEVATION>()) as *mut c_void };
        let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
        let mut ret_size = size;
        unsafe {
            GetTokenInformation(
                &handle,
                TokenElevation,
                elevation,
                size as u32,
                &mut ret_size,
            )
        };
        let elevation_struct: TOKEN_ELEVATION = unsafe{ *(elevation as *mut TOKEN_ELEVATION)};

        if !handle.is_invalid() {
            unsafe {
                CloseHandle(&handle);
            }
        }

        elevation_struct.TokenIsElevated == 1
    }
}

/// Determines privilege levels
pub async fn handle() -> Result<String, Box<dyn Error>> {
    // TODO: Implement Linux check
    let is_admin = is_elevated();  
    notion_out!("Admin Context: ", &is_admin.to_string())  
}