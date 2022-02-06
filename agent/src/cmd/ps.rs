use std::error::Error;
use sysinfo::{ProcessExt, System, SystemExt};

pub async fn handle() -> Result<String, Box<dyn Error>> {
    let mut process_res = String::new();
    let sys = System::new_all();
    for (pid, process) in sys.processes() {
        process_res.push_str(format!("{} {}\n", pid, process.name()).as_str());
    }
    Ok(process_res)
}