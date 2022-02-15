extern crate embed_resource;
use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os {
        Ok(o) => {
            if o == "windows" {
                println!("Building for windows!");
                embed_resource::compile("offensive_notion.rc");
            } else {
                println!("Building for Linux!");
            }
        },
        Err(e) => {println!("{e}")}
    }
}

