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

pub fn get_module_handle(module_name: &[u8]) -> Result<HMODULE, u32> {
    let mh = unsafe {
        libloaderapi::GetModuleHandleA(module_name.as_ptr() as *const _)
    };
    if mh.is_null() {
        Err(unsafe { winapi::um::errhandlingapi::GetLastError() })
    } else {
        Ok(mh)
    }
}

pub fn get_proc_address(module_handle: HMODULE, proc_name: &[u8]) -> Result<FARPROC, u32> {
    let pa = unsafe {
        libloaderapi::GetProcAddress(module_handle, proc_name.as_ptr() as *const _)
    };

    if pa.is_null() {
        Err(unsafe { winapi::um::errhandlingapi::GetLastError() })
    } else {
        Ok(pa)
    }
}

pub fn create_remote_thread(process_handle: HANDLE, start_addr: u32, param: u32) -> Result<HANDLE, u32> {
    let thread = unsafe {
        processthreadsapi::CreateRemoteThread(
            process_handle as HANDLE,
            ptr::null_mut(),
            0,
            Some(mem::transmute(start_addr as usize)),
            param as *mut _,
            0,
            ptr::null_mut(),
        )
    };
    if thread.is_null() {
        Err(unsafe { winapi::um::errhandlingapi::GetLastError() })
    } else {
        Ok(thread)
    }
}
