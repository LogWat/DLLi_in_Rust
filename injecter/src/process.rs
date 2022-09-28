use std::{mem, ptr};
use winapi::{
    um::{
        winnt::{HANDLE, PROCESS_ALL_ACCESS, MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE},
        processthreadsapi, handleapi, errhandlingapi, psapi, memoryapi,
        tlhelp32,
        tlhelp32::{
            PROCESSENTRY32W, TH32CS_SNAPPROCESS,
        }
    },
    shared::{
        minwindef::{MAX_PATH},
    }
};

#[allow(dead_code)]
pub enum MemSize {
    Byte(u8),
    Word(u16),
    Dword(u32),
    Qword(u64),
}

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

    pub fn name(&self) -> String {
        let mut name = [0u16; MAX_PATH];

        unsafe {
            psapi::GetModuleBaseNameW(
                self.handle,
                std::ptr::null_mut(),
                name.as_mut_ptr(),
                MAX_PATH as u32,
            );
        }

        String::from_utf16_lossy(&name)
    }

    pub fn alloc_mem(&self, size: usize) -> Result<u32, u32> {
        let addr = unsafe {
            memoryapi::VirtualAllocEx(
                self.handle,
                std::ptr::null_mut(),
                size,
                MEM_COMMIT | MEM_RELEASE,
                PAGE_READWRITE,
            )
        };
        if addr.is_null() {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(addr as u32)
    }

    pub fn write_mem(&self, address: usize, data: &str) -> Result<(), u32> {
        let data = data.as_bytes();
        if data.len() == 0 {
            return Ok(());
        }

        if unsafe {
            memoryapi::WriteProcessMemory(
                self.handle,
                address as *mut _,
                data.as_ptr() as *const _,
                data.len(),
                ptr::null_mut(),
            )
        } == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(())
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