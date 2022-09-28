mod process;

fn main() {
    print_process();
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