use std::{mem, ptr};
use winapi::{
    um::{
        winnt::{HANDLE, PROCESS_ALL_ACCESS},
        processthreadsapi, handleapi, errhandlingapi,
        tlhelp32,
        tlhelp32::{
            PROCESSENTRY32W, THREADENTRY32, TH32CS_SNAPPROCESS, TH32CS_SNAPTHREAD,
        }
    },
};

pub struct Process {
    pub handle: HANDLE,
    pub pid: u32,
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            handleapi::CloseHandle(self.handle);
        }
    }
}

impl Process {
    pub fn new(pid: u32) -> Self {
        let handle = unsafe {
            processthreadsapi::OpenProcess(PROCESS_ALL_ACCESS, 0, pid)
        };

        Self {
            handle,
            pid,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.handle != std::ptr::null_mut()
    }
}

pub fn enumerate_process() -> Result<Vec<Process>, u32> {
    let mut prs: Vec<Process> = Vec::new();
    let mut psetry: PROCESSENTRY32W = unsafe { mem::zeroed() };
    psetry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;

    let snap = unsafe {
        tlhelp32::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
    };

    if snap == ptr::null_mut() {
        return Err(unsafe { errhandlingapi::GetLastError() });
    }

    let mut ret = unsafe {
        tlhelp32::Process32FirstW(snap, &mut psetry)
    };
    while ret != 0 {
        prs.push(Process::new(psetry.th32ProcessID));
        ret = unsafe {
            tlhelp32::Process32NextW(snap, &mut psetry)
        };
    }

    unsafe {
        handleapi::CloseHandle(snap);
    }

    Ok(prs)
}