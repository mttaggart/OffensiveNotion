#[cfg(windows)] extern crate windres;
#[cfg(windows)] use windres::Build;

#[cfg(windows)]
fn main() {
    if cfg!(windows) {
        println!("Building for windows!");
        Build::new().compile("offensive_notion.rc").unwrap();
    }
}

#[cfg(not(windows))]
fn main() {
    println!("Building for Linux!");
}