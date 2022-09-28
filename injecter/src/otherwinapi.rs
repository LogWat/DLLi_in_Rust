use std::{ptr, mem};
use winapi::{
    um::{
        winnt::{HANDLE},
        libloaderapi, processthreadsapi,
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

pub fn create_remote_thread(process_handle: *mut u8, start_addr: *mut u8, param: *mut u8) -> Result<*mut u8, u32> {
    let thread = unsafe {
        processthreadsapi::CreateRemoteThread(
            process_handle as HANDLE,
            ptr::null_mut(),
            0,
            Some(mem::transmute(start_addr)),
            param as *mut _,
            0,
            ptr::null_mut(),
        )
    };
    if thread.is_null() {
        Err(unsafe { winapi::um::errhandlingapi::GetLastError() })
    } else {
        Ok(thread as *mut u8)
    }
}