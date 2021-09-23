extern crate user32;
extern crate winapi;
extern crate kernel32;

use kernel32::{
    OpenProcess,
    CloseHandle
};
use winapi::{
    um::{
        winnt::{
            PROCESS_CREATE_THREAD, // CreateRemoteThread
            PROCESS_VM_OPERATION,  // VirtualAllocEx
            PROCESS_VM_WRITE       // WriteProcessMemory
        },
        errhandlingapi::{
            GetLastError
        }
    },
    shared::minwindef::{
        DWORD, 
        FALSE,
        BOOL
    }
};

fn main() {
    let word = input();
    let pid: u32 = word.parse().unwrap();
    println!("{}", pid);

    unsafe {
        let process = Some(OpenProcess(
            PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE, 
            FALSE, 
            pid)).unwrap_or_else(|| panic!("{}", GetLastError()));

        CloseHandle(process);
    }
}

fn input() -> String {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    word.trim().to_string()
}