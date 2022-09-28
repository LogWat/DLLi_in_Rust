use winapi::{
    um::{
        winnt::{HANDLE, PROCESS_ALL_ACCESS},
        processthreadsapi,
    },
};

pub struct Process {
    pub handle: HANDLE,
    pub pid: u32,
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
}