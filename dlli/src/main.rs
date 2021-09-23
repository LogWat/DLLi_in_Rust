extern crate user32;
extern crate winapi;
extern crate kernel32;

use kernel32::OpenProcess;
use winapi::{
    um::{
        winnt::{
            PROCESS_ALL_ACCESS
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
    let processid: DWORD = 10000;
    unsafe {
        let process = OpenProcess(
            PROCESS_ALL_ACCESS, 
            FALSE, 
            processid).as_ref().expect(GetLastError());
    }
}
