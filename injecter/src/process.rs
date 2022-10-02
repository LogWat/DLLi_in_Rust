use std::{mem, ptr};
use winapi::{
    um::{
        winnt::{HANDLE, PROCESS_ALL_ACCESS, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE},
        processthreadsapi, handleapi, errhandlingapi, psapi, memoryapi,
        tlhelp32,
        tlhelp32::{
            PROCESSENTRY32W, TH32CS_SNAPPROCESS,
        }
    },
    shared::{
        minwindef::{MAX_PATH}
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
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.handle != ptr::null_mut()
    }


    pub fn new(pid: u32) -> Result<Self, u32> {
        let handle = unsafe {
            processthreadsapi::OpenProcess(PROCESS_ALL_ACCESS, 0, pid)
        };
        if handle == ptr::null_mut() {
            return Err(unsafe{ errhandlingapi::GetLastError() });
        }

        Ok(Process {
            handle,
            pid,
        })
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

    pub fn alloc_mem(&self, size: u32) -> Result<u32, u32> {
        let addr = unsafe {
            memoryapi::VirtualAllocEx(
                self.handle,
                0 as _,
                size as _,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        };
        if addr.is_null() {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(addr as u32)
    }

    pub fn write_mem(&self, address: u32, data: &str) -> Result<(), u32> {
        let data = data.as_bytes();
        if data.len() == 0 {
            return Ok(());
        }

        if unsafe {
            memoryapi::WriteProcessMemory(
                self.handle,
                address as _,
                data.as_ptr() as _,
                data.len() as _,
                0 as _,
            )
        } == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn change_protection(&self, address: u32, size: u32, protection: u32) -> Result<u32, u32> {
        let mut old_protection = 0;
        if unsafe {
            memoryapi::VirtualProtectEx(
                self.handle,
                address as _,
                size as _,
                protection,
                &mut old_protection,
            )
        } == 0 {
            return Err(unsafe { errhandlingapi::GetLastError() });
        }

        Ok(old_protection)
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
        let p = match Process::new(psetry.th32ProcessID) {
            Ok(p) => p,
            Err(_) => {
                ret = unsafe {
                    tlhelp32::Process32NextW(snap, &mut psetry)
                };
                continue;
            }
        };
        prs.push(p);
        ret = unsafe {
            tlhelp32::Process32NextW(snap, &mut psetry)
        };
    }

    unsafe {
        handleapi::CloseHandle(snap);
    }

    Ok(prs)
}