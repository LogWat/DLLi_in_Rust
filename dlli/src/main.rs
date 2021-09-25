extern crate user32;
extern crate winapi;
extern crate kernel32;
extern crate w32_error;
extern crate libc;

use kernel32::{
    OpenProcess,
    CloseHandle
};
use winapi::{
    um::{
        winnt::{
            PROCESS_CREATE_THREAD, // CreateRemoteThread
            PROCESS_VM_OPERATION,  // VirtualAllocEx
            PROCESS_VM_WRITE,      // WriteProcessMemory
        },
        errhandlingapi::{
            GetLastError
        }
    },
    shared::{
        minwindef::{
            FALSE,
            DWORD
        },
        /*ntdef::{
            HANDLE
        }*/
    }
};
use w32_error::W32Error;

type HANDLE = *mut libc::c_void;


fn main() {
    let word = input();
    let pid: DWORD = word.parse().unwrap();

    unsafe {
        let process = winapi_openprocess(pid).unwrap();

        println!("{:?}", process);

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