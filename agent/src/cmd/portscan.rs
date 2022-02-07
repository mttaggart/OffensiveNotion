use std::error::Error;

/// Scans target IP/CIDR for open ports
pub async fn handle(_s: &String) -> Result<String, Box<dyn Error>> {
    // Ref: https://github.com/skerkour/kerkour.com/tree/main/2021/rust_fast_port_scanner
    // TODO
    Ok("Not implemented yet!".to_string())
}