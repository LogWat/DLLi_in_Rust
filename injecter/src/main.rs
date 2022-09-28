mod process;
mod otherwinapi;

fn main() {
    print_process();
    let mut s = String::new();
    println!("Enter the pid of the process you want to inject into:");
    std::io::stdin().read_line(&mut s).unwrap();
    let pid = s.trim().parse::<u32>().unwrap();
    println!("Enter the path to the dll you want to inject:");
    std::io::stdin().read_line(&mut s).unwrap();
    let dll_path = s.trim();
    let _rthread = match inject_dll(pid, dll_path) {
        Ok(rthread) => rthread,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}

fn print_process() {
    let processes = match process::enumerate_process() {
        Ok(processes) => processes,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };
    println!("==============================");
    for process in processes {
        let name = process.name();
        println!("PID: {:6} Name: {}", process.pid, name);
    }
    println!("==============================");
}

fn inject_dll(pid: u32, dll_path: &str) -> Result<*mut u8, String> {
    let process = process::Process::new(pid);
    let dll_path = match std::fs::canonicalize(dll_path) {
        Ok(path) => path,
        Err(error) => {
            return Err(format!("Error: {}", error));
        }
    };
    let dll_path = match dll_path.to_str() {
        Some(path) => path,
        None => {
            return Err("Error: Invalid path".to_string());
        }
    };
    let mem = match process.alloc_mem(dll_path.len() + 1) {
        Ok(mem) => mem,
        Err(error) => {
            return Err(format!("Failed to allocate memory. Code: {}", error));
        }
    };
    match process.write_mem(mem as usize, dll_path) {
        Ok(_) => (),
        Err(error) => {
            return Err(format!("Failed to write memory. Code: {}", error));
        }
    };

    let kernel32 = match otherwinapi::get_module_handle("kernel32.dll\0") {
        Ok(handle) => handle,
        Err(error) => {
            return Err(format!("Error: Failed GetModuleHandle. Code: {}", error));
        }
    };

    let load_lib = match otherwinapi::get_proc_address(kernel32, "LoadLibraryA\0") {
        Ok(handle) => handle,
        Err(error) => {
            return Err(format!("Error: Failed GetProcAddress. Code: {}", error));
        }
    };

    let thread = match otherwinapi::create_remote_thread(
        process.handle as *mut u8,
        load_lib,
        mem as *mut u8,
    ) {
        Ok(thread) => thread,
        Err(error) => {
            return Err(format!("Error: Failed CreateRemoteThread. Code: {}", error));
        }
    };

    Ok(thread)
}