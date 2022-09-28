use winapi::{
    um::{
        winnt::{},
        libloaderapi,
    },
    shared::{
        minwindef::*,
    }
};

pub fn get_module_handle(module_name: &str) -> Result<*mut u8, u32> {
    let mh = unsafe {
        libloaderapi::GetModuleHandleA(module_name.as_ptr() as *const i8)
    };
    if mh.is_null() {
        Err(unsafe { winapi::um::errhandlingapi::GetLastError() })
    } else {
        Ok(mh as *mut u8)
    }
}

pub fn get_proc_address(module_handle: *mut u8, proc_name: &str) -> Result<*mut u8, u32> {
    let pa = unsafe {
        libloaderapi::GetProcAddress(module_handle as *mut HINSTANCE__, proc_name.as_ptr() as *const i8)
    };

    if pa.is_null() {
        Err(unsafe { winapi::um::errhandlingapi::GetLastError() })
    } else {
        Ok(pa as *mut u8)
    }
}