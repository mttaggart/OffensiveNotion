use std::error::Error;
use std::process::Command;

pub async fn handle() -> Result<String, Box<dyn Error>> {
// This is a lame kludge because getting process data is tough, but at least
// it's ergonomic?
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "tasklist"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .args(["ps", "aux"])
            .output()
            .expect("failed to execute process")
    };
    let output_string: String;
    if output.stderr.len() > 0 {
        output_string = String::from_utf8(output.stderr).unwrap();
    } else {
        output_string = String::from_utf8(output.stdout).unwrap();
    }
    return Ok(output_string);
}