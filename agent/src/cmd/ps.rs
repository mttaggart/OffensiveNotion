use std::error::Error;
use crate::cmd::shell;

pub async fn handle() -> Result<String, Box<dyn Error>> {
// This is a lame kludge because getting process data is tough, but at least
// it's ergonomic?
    #[cfg(windows)] {
        shell::handle("tasklist".to_string()).await
    }

    #[cfg(not(windows))] {
        shell::handle(&"ps aux".to_string()).await
    }
}