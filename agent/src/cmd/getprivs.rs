use std::error::Error;
use is_root::is_root;

#[cfg(windows)] use std::ptr::null_mut;
#[cfg(windows)] use winapi::um::handleapi::CloseHandle;
#[cfg(windows)] use winapi::um::processthreadsapi::GetCurrentProcess;
#[cfg(windows)] use winapi::um::processthreadsapi::OpenProcessToken;
#[cfg(windows)] use winapi::um::securitybaseapi::GetTokenInformation;
#[cfg(windows)] use winapi::um::winnt::TokenElevation;
#[cfg(windows)] use winapi::um::winnt::HANDLE;
#[cfg(windows)] use winapi::um::winnt::TOKEN_ELEVATION;
#[cfg(windows)] use libc;
#[cfg(windows)] use std::mem;
#[cfg(windows)] use winapi::ctypes::c_void;
#[cfg(windows)] use winapi::um::winnt::TOKEN_QUERY;


pub fn is_elevated() -> bool {
    
    //TODO: parameterize for Linux/Windows
    //On Linux, check UID/EUID for 0
    #[cfg(not(windows))] {
        let is_root = is_root();
        return is_root;
    }

    #[cfg(windows)] {
    let mut handle: HANDLE = null_mut();
    unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) };

    let elevation = unsafe { libc::malloc(mem::size_of::<TOKEN_ELEVATION>()) as *mut c_void };
    let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
    let mut ret_size = size;
    unsafe {
        GetTokenInformation(
            handle,
            TokenElevation,
            elevation,
            size as u32,
            &mut ret_size,
        )
    };
    let elevation_struct: TOKEN_ELEVATION = unsafe{ *(elevation as *mut TOKEN_ELEVATION)};

    if !handle.is_null() {
        unsafe {
            CloseHandle(handle);
        }
    }

    elevation_struct.TokenIsElevated == 1
}
}

/// Determines privilege levels
pub async fn handle() -> Result<String, Box<dyn Error>> {
    // TODO: Implement Linux check
        let is_admin = is_elevated();  
        println!("{}", is_admin);
        Ok(String::from(format!("Admin Context: {is_admin}").to_string()))
    
}