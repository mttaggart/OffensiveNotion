use std::error::Error;

pub async fn handle(s: &String) -> Result<String, Box<dyn Error>> {
    // TODO: Implement
    #[cfg(windows)] {
        return Ok(String::from("Under Construction!"))
    }
    #[cfg(not(windows))] {
        return Ok(String::from("Runas only works on Windows!"))
    }
}