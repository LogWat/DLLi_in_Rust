extern crate user32;
extern crate winapi;
extern crate kernel32;

use kernel32::OpenProcess;
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
        DWORD, FALSE
    }
};

fn main() {
    let word = input();
    let pid: u32 = word.parse().unwrap();
    println!("{}", pid);

    unsafe {
        let process = OpenProcess(
            PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE, 
            FALSE, 
            pid).as_ref().unwrap_or_else(|| panic!("{}", GetLastError()));
    }
}

fn input() -> String {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    word.trim().to_string()
}