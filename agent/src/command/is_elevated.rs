// Ref: https://users.rust-lang.org/t/how-do-i-determine-if-i-have-admin-rights-on-windows/35710/5

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

#[cfg(windows)]
pub fn is_elevated() -> bool {
    
    //TODO: parameterize for Linux/Windows
    //On Linux, check UID/EUID for 0

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