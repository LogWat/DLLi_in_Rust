extern crate user32;
extern crate winapi;
extern crate kernel32;
extern crate w32_error;
extern crate libc;

use kernel32::{
    OpenProcess,
    VirtualAllocEx,
    CloseHandle
};
use winapi::{
    um::{
        winnt::{
            PROCESS_CREATE_THREAD, // CreateRemoteThread
            PROCESS_VM_OPERATION,  // VirtualAllocEx
            PROCESS_VM_WRITE,      // WriteProcessMemory
            MEM_COMMIT,
            PAGE_READWRITE,
        },
        errhandlingapi::{
            GetLastError
        }
    },
    shared::{
        minwindef::{
            FALSE,
            DWORD,
            LPVOID,
        },
        ntdef::{
            PVOID
        }
    }
};
use w32_error::W32Error;

type HANDLE = *mut libc::c_void;

fn main() {
    let word = input();
    let pid: DWORD = word.parse().unwrap();

    unsafe {
        let process = winapi_openprocess(pid).unwrap();

        let path_name: String = "inject.dll".to_string();
        let path_size = path_name.len() as u64;

        let rlp = winapi_VirtualAllocEx(process, path_size);

        CloseHandle(process);
    }
}

fn input() -> String {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    word.trim().to_string()
}

unsafe fn winapi_openprocess(pid: DWORD) -> Result<HANDLE, W32Error> {

    let result = OpenProcess(
        PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE, 
        FALSE, 
        pid);
    
    if result as usize == 0 {
        Err(W32Error::new(0))
    } else {
        Ok(result)
    }
}

unsafe fn winapi_VirtualAllocEx(proc: HANDLE, pathsize: u64) -> Result<HANDLE, W32Error> {

    let result = VirtualAllocEx(
        proc, 
        0 as *mut libc::c_void,
        pathsize, 
        MEM_COMMIT, 
        PAGE_READWRITE);

    if result as usize == 0 {
        Err(W32Error::new(0))
    } else {
        Ok(result)
    }
}